use std::io::{Read, Result};
use std::fs::{File};
use std::error::Error;
use std::path::{Path, PathBuf};
use zeal::system_definition::*;

#[derive(PartialEq, Copy, Clone)]
pub struct NumberLiteral {
    pub number: u32,
    pub argument_size: ArgumentSize,
}

#[derive(Clone, PartialEq)]
pub enum TokenType {
    Invalid(char),
    Identifier(String),
    Opcode(String),
    NumberLiteral(NumberLiteral),
    StringLiteral(String),
    Register(String),
    Comma,
    Immediate,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    Colon,
    EndOfFile,
    KeywordInclude,
    KeywordIncbin,
    KeywordOrigin,
    KeywordSnesMap,
}

#[derive(Clone)]
pub struct Token {
    pub ttype: TokenType,
    pub line: u32,
    pub start_column: u32,
    pub end_column: u32,
    pub source_file: String,
    pub context_start: usize
}

pub struct Lexer {
    system: &'static SystemDefinition,
    pub source_file: String,
    file_content: Vec<char>,
    current_char: usize,
    line: u32,
    column: u32,
    line_start: usize
}

fn is_ascii_numeric(current_char: char) -> bool {
    current_char >= '0' && current_char <= '9'
}

fn is_ascii_binary_digit(current_char: char) -> bool {
    current_char == '0' || current_char == '1'
}

fn is_ascii_hex_digit(current_char: char) -> bool {
    is_ascii_numeric(current_char) || (current_char >= 'a' && current_char <= 'f')
        || (current_char >= 'A' && current_char <= 'F')
}

fn is_ascii_alphanumeric(current_char: char) -> bool {
    is_ascii_numeric(current_char) || (current_char >= 'A' && current_char <= 'Z')
        || (current_char >= 'a' && current_char <= 'z')
}

fn absolute_path(path: &Path) -> Result<PathBuf> {
    let path_buf = path.canonicalize()?;

    #[cfg(windows)]
    let path_buf = Path::new(
        path_buf
            .as_path()
            .to_string_lossy()
            .trim_left_matches(r"\\?\"),
    ).to_path_buf();

    Ok(path_buf)
}

impl Lexer {
    // pub fn from_string(
    //     system: &'static SystemDefinition,
    //     file_content: &str,
    // ) -> Self {
    //     Lexer {
    //         system: system,
    //         file_content: file_content.chars().collect(),
    //         current_char: 0,
    //         source_file: String::from(""),
    //         line: 1,
    //         column: 1,
    //         line_start: 0,
    //     }
    // }

    pub fn from_file(system: &'static SystemDefinition, filename: &str) -> Self {
        let input_path = Path::new(filename);
        let path_display = input_path.display();

        let mut file = match File::open(input_path) {
            Err(why) => panic!("Couldn't open {}: {}", path_display, why.description()),
            Ok(file) => file,
        };

        let mut string_file_content = String::new();
        match file.read_to_string(&mut string_file_content) {
            Err(why) => panic!("Couldn't read {}: {}", path_display, why.description()),
            Ok(result) => result,
        };

        let absolute_path_buf = match absolute_path(input_path) {
            Err(_) => None,
            Ok(result) => {
                Some(result)
            }
        };

        Lexer {
            system: system,
            file_content: string_file_content.chars().collect(),
            current_char: 0,
            source_file: absolute_path_buf.unwrap().to_str().unwrap().to_string(),
            line: 1,
            column: 1,
            line_start: 0,
        }
    }

    pub fn get_next_token(&mut self) -> Token {
        self.eat_whitespaces();
        self.eat_comment();

        match self.peek() {
            None => self.token_eof(),
            Some(&current_char) => self.parse_token(current_char),
        }
    }

    pub fn reset(&mut self) {
        self.line = 1;
        self.column = 0;
        self.current_char = 0;
        self.line_start = 0;
    }

    pub fn lookahead(&mut self, times: u32) -> Token {
        let backup_line = self.line;
        let backup_column = self.column;
        let backup_current_char = self.current_char;
        let backup_line_start = self.line_start;

        for _i in 0..(times - 1) {
            self.get_next_token();
        }

        let lookahead = self.get_next_token();

        self.line = backup_line;
        self.column = backup_column;
        self.current_char = backup_current_char;
        self.line_start = backup_line_start;

        return lookahead;
    }

    fn parse_token(&mut self, current_char: char) -> Token {
        match current_char {
            'a'...'z' | 'A'...'Z' | '_' => {
                return self.parse_identifier_or_similar();
            }
            '"' => {
                return self.parse_string_literal();
            }
            '#' => {
                return self.new_simple_token(TokenType::Immediate);
            }
            '$' => {
                return self.parse_hex_number();
            }
            ',' => {
                return self.new_simple_token(TokenType::Comma);
            }
            '(' => {
                return self.new_simple_token(TokenType::LeftParen);
            }
            ')' => {
                return self.new_simple_token(TokenType::RightParen);
            }
            '[' => {
                return self.new_simple_token(TokenType::LeftBracket);
            }
            ']' => {
                return self.new_simple_token(TokenType::RightBracket);
            }
            '%' => {
                return self.parse_binary_number();
            }
            ':' => {
                return self.new_simple_token(TokenType::Colon);
            }
            _ => if is_ascii_numeric(current_char) {
                return self.parse_number();
            } else {
                return self.token_invalid();
            },
        }
    }

    fn eat_whitespaces(&mut self) {
        while let Some(&current_char) = self.peek() {
            if current_char == '\n' {
                self.do_end_of_line();
            } else if !current_char.is_whitespace() {
                break;
            } else {
                self.consume();
            }
        }
    }

    fn eat_comment(&mut self) {
        let mut is_done = false;
        while !is_done {
            match self.peek() {
                Some(&first_char) => if first_char == '/' {
                    match self.peek_lookahead(1) {
                        Some(second_char) => if second_char == '/' {
                            while let Some(&current_char) = self.peek() {
                                if current_char == '\n' {
                                    self.do_end_of_line();
                                    break;
                                } else {
                                    self.consume();
                                }
                            }
                        } else {
                            is_done = true
                        },
                        None => is_done = true,
                    }
                } else {
                    is_done = true
                },
                None => is_done = true,
            };
        }
        self.eat_whitespaces();
    }

    fn parse_identifier_or_similar(&mut self) -> Token {
        let context_start = self.line_start;
        let start_column = self.column;
        let mut parsed_identifier = String::new();

        parsed_identifier.push(self.consume().unwrap());

        loop {
            match self.peek() {
                None => break,
                Some(&current_char) => {
                    if is_ascii_alphanumeric(current_char) || current_char == '_' {
                        parsed_identifier.push(self.consume().unwrap())
                    } else {
                        break;
                    }
                }
            }
        }

        let end_column = self.column;

        match self.is_keyword(&parsed_identifier) {
            Some(keyword) => {
                return Token {
                    ttype: keyword,
                    line: self.line,
                    start_column: start_column,
                    end_column: end_column,
                    source_file: self.source_file.to_string(),
                    context_start: context_start,
                };
            }
            None => if self.is_opcode(&parsed_identifier) {
                return Token {
                    ttype: TokenType::Opcode(parsed_identifier),
                    line: self.line,
                    start_column: start_column,
                    end_column: end_column,
                    source_file: self.source_file.to_string(),
                    context_start: context_start,
                };
            } else if self.is_register(&parsed_identifier) {
                return Token {
                    ttype: TokenType::Register(parsed_identifier),
                    line: self.line,
                    start_column: start_column,
                    end_column: end_column,
                    source_file: self.source_file.to_string(),
                    context_start: context_start,
                };
            } else {
                return Token {
                    ttype: TokenType::Identifier(parsed_identifier),
                    line: self.line,
                    start_column: start_column,
                    end_column: end_column,
                    source_file: self.source_file.to_string(),
                    context_start: context_start,
                };
            },
        }
    }

    fn parse_string_literal(&mut self) -> Token {
        let context_start = self.line_start;
        let start_column = self.column;

        let mut parsed_string = String::new();

        // Eat first '"'
        self.consume();

        loop {
            match self.peek() {
                None => break,
                Some(&current_char) => {
                    if current_char != '"' {
                        parsed_string.push(self.consume().unwrap())
                    } else {
                        break;
                    }
                }
            }
        }

        let end_lookahead = self.peek_lookahead(0);
        match end_lookahead {
            Some(result) => {
                if result == '"' {
                    self.consume();

                    let end_column = self.column;

                    return Token {
                        ttype: TokenType::StringLiteral(parsed_string),
                        line: self.line,
                        start_column: start_column,
                        end_column: end_column,
                        source_file: self.source_file.to_string(),
                        context_start: context_start,
                    };
                } else {
                    self.token_invalid()
                }
            }
            None => {
                self.token_invalid()
            }
        }
    }

    fn is_keyword(&mut self, identifier: &str) -> Option<TokenType> {
        match identifier {
            "include" => Some(TokenType::KeywordInclude),
            "incbin" => Some(TokenType::KeywordIncbin),
            "origin" => Some(TokenType::KeywordOrigin),
            "snesmap" => Some(TokenType::KeywordSnesMap),
            _ => None,
        }
    }

    fn parse_hex_number(&mut self) -> Token {
        let context_start = self.line_start;
        let start_column = self.column;

        // Eat $
        self.consume();

        let mut parsed_number = String::new();

        loop {
            match self.peek() {
                None => break,
                Some(&current_char) => if is_ascii_hex_digit(current_char) {
                    parsed_number.push(self.consume().unwrap())
                } else {
                    break;
                },
            }
        }

        let end_column = self.column;

        let result_number = match u32::from_str_radix(&parsed_number, 16) {
            Ok(result) => result,
            Err(_) => 0,
        };

        let parsed_length = parsed_number.len();

        let argument_size = if parsed_length > 6 {
            ArgumentSize::Word32
        } else if parsed_length > 4 {
            ArgumentSize::Word24
        } else if parsed_length > 2 {
            ArgumentSize::Word16
        } else {
            ArgumentSize::Word8
        };

        let number_literal = NumberLiteral {
            number: result_number,
            argument_size: argument_size,
        };

        self.new_token(
            TokenType::NumberLiteral(number_literal),
            start_column,
            end_column,
            context_start,
        )
    }

    fn parse_binary_number(&mut self) -> Token {
        let context_start = self.line_start;
        let start_column = self.column;

        // Eat %
        self.consume();

        let mut parsed_number = String::new();

        loop {
            match self.peek() {
                None => break,
                Some(&current_char) => if is_ascii_binary_digit(current_char) {
                    parsed_number.push(self.consume().unwrap())
                } else {
                    break;
                },
            }
        }

        let end_column = self.column;

        let result_number = match u32::from_str_radix(&parsed_number, 2) {
            Ok(result) => result,
            Err(_) => 0,
        };

        let parsed_length = parsed_number.len();

        let argument_size = if parsed_length > 24 {
            ArgumentSize::Word32
        } else if parsed_length > 16 {
            ArgumentSize::Word24
        } else if parsed_length > 8 {
            ArgumentSize::Word16
        } else {
            ArgumentSize::Word8
        };

        let number_literal = NumberLiteral {
            number: result_number,
            argument_size: argument_size,
        };

        self.new_token(
            TokenType::NumberLiteral(number_literal),
            start_column,
            end_column,
            context_start,
        )
    }

    fn parse_number(&mut self) -> Token {
        let context_start = self.line_start;
        let start_column = self.column;
        let mut parsed_number = String::new();

        parsed_number.push(self.consume().unwrap());

        loop {
            match self.peek() {
                None => break,
                Some(&current_char) => if is_ascii_numeric(current_char) {
                    parsed_number.push(self.consume().unwrap())
                } else {
                    break;
                },
            }
        }

        let end_column = self.column;

        let result_number = match u32::from_str_radix(&parsed_number, 10) {
            Ok(result) => result,
            Err(_) => 0,
        };

        let argument_size = number_to_argument_size(result_number);

        let number_literal = NumberLiteral {
            number: result_number,
            argument_size: argument_size,
        };

        self.new_token(
            TokenType::NumberLiteral(number_literal),
            start_column,
            end_column,
            context_start,
        )
    }

    fn is_opcode(&self, identifier: &str) -> bool {
        for instruction in self.system.instructions.iter() {
            if instruction.name == identifier {
                return true;
            }
        }

        return false;
    }

    fn is_register(&self, identifier: &str) -> bool {
        for &register in self.system.registers.iter() {
            if register == identifier {
                return true;
            }
        }

        return false;
    }

    fn do_end_of_line(&mut self) {
        self.line += 1;
        self.column = 0;

        self.consume();
        self.line_start = self.current_char;
    }

    fn token_invalid(&mut self) -> Token {
        let context_start = self.line_start;

        let invalid_char = match self.consume() {
            Some(result) => result,
            None => ' ',
        };

        let start_column = self.column - 1;
        let end_column = self.column;

        self.new_token(
            TokenType::Invalid(invalid_char),
            start_column,
            end_column,
            context_start,
        )
    }

    fn token_eof(&mut self) -> Token {
        let start_column = self.column;
        let end_column = self.column;
        let context_start = self.line_start;

        self.new_token(
            TokenType::EndOfFile,
            start_column,
            end_column,
            context_start,
        )
    }

    fn new_simple_token(&mut self, ttype: TokenType) -> Token {
        let context_start = self.line_start;
        let start_column = self.column;
        self.consume();
        let end_column = self.column;
        return self.new_token(ttype, start_column, end_column, context_start);
    }

    fn new_token(
        &mut self,
        ttype: TokenType,
        start_column: u32,
        end_column: u32,
        context_start: usize,
    ) -> Token {
        Token {
            ttype: ttype,
            line: self.line,
            start_column: start_column,
            end_column: end_column,
            source_file: self.source_file.to_string(),
            context_start: context_start,
        }
    }

    fn peek(&mut self) -> Option<&char> {
        if self.current_char < self.file_content.len() {
            return Some(&self.file_content[self.current_char]);
        }
        else {
            return None;
        }
    }

    fn peek_lookahead(&mut self, lookahead: usize) -> Option<char> {
        let lookahead = self.current_char + lookahead;

        if lookahead < self.file_content.len() {
            return Some(self.file_content[lookahead]);
        } else {
            return None;
        }
    }

    fn consume(&mut self) -> Option<char> {
        if self.current_char< self.file_content.len() {
            let consumed_char = self.file_content[self.current_char];
            self.current_char += 1;
            self.column += 1;
            return Some(consumed_char);
        } 
        else {
            return None;
        }
    }
}
