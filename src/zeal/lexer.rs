use std::str::Chars;
use std::iter::Peekable;
use zeal::system_definition::*;

#[derive(PartialEq, Copy, Clone)]
pub enum ArgumentSize {
    Word8,
    Word16,
    Word24,
    Word32,
}

#[derive(PartialEq, Copy, Clone)]
pub struct NumberLiteral {
    pub number: u32,
    pub argument_size: ArgumentSize
}

#[derive(PartialEq)]
pub enum TokenType {
    Invalid(char),
    Identifier(String),
    Opcode(String),
    NumberLiteral(NumberLiteral),
    Immediate,
    EndOfFile,
}

pub struct Token {
    pub ttype: TokenType,
    pub line: u32,
    pub start_column: u32,
    pub end_column: u32,
    pub source_file: String,
}

pub struct Lexer<'a> {
    system: &'static SystemDefinition,
    it: Peekable<Chars<'a>>,
    pub start_line: Peekable<Chars<'a>>,
    source_file: String,
    line: u32,
    column: u32,
}

fn is_ascii_numeric(current_char: char) -> bool {
    current_char >= '0' && current_char <= '9'
}

fn is_ascii_binary_digit(current_char: char) -> bool {
    current_char == '0' || current_char == '1'
}

fn is_ascii_hex_digit(current_char: char) -> bool {
    is_ascii_numeric(current_char)
    || (current_char >= 'a' && current_char <= 'f')
    || (current_char >= 'A' && current_char <= 'F')
}

fn is_ascii_alphanumeric(current_char: char) -> bool {
    is_ascii_numeric(current_char)
    || (current_char >= 'A' && current_char <= 'Z')
    || (current_char >= 'a' && current_char <= 'z')
}

impl<'a> Lexer<'a> {
    pub fn new(system: &'static SystemDefinition, file_content: &'a str, source_file: String) -> Self {
        Lexer{
            system: system,
            it: file_content.chars().peekable(),
            source_file: source_file,
            start_line: file_content.chars().peekable(),
            line: 1,
            column: 1,
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

    pub fn lookahead(&mut self) -> Token {
        let backup_line = self.line;
        let backup_column = self.column;
        let backup_it = self.it.clone();

        let lookahead = self.get_next_token();

        self.line = backup_line;
        self.column = backup_column;
        self.it = backup_it;

        return lookahead;
    }

    fn parse_token(&mut self, current_char: char) -> Token {
        match current_char {
            'a'...'z' | 'A'...'Z' | '_' => {
                return self.parse_identifier_or_keyword_or_opcode();
            },
            '#' => {
                let start_column = self.column;
                self.consume();
                let end_column = self.column;
                return self.new_token(TokenType::Immediate, start_column, end_column);
            },
            '$' => {
                return self.parse_hex_number();
            },
            '%' => {
                return self.parse_binary_number();
            },
            _ => {
                if is_ascii_numeric(current_char) {
                    return self.parse_number();
                }
                else {
                    return self.token_invalid();
                }
            }
        }
    }

    fn eat_whitespaces(&mut self) {
        while let Some(&current_char) = self.peek() {
            if current_char == '\n' {
                self.do_end_of_line();
            }
            else if !current_char.is_whitespace() {
                break;
            } else {
                self.consume();
            }
        }
    }

    fn eat_comment(&mut self) {
        match self.peek() {
            Some(&first_char) => {
                if first_char == '/' {
                    match self.peek_lookahead(1) {
                        Some(second_char) => {
                            if second_char == '/' {
                                while let Some(&current_char) = self.peek() {
                                    if current_char == '\n' {
                                        self.do_end_of_line();
                                        break;
                                    } else {
                                        self.consume();
                                    }
                                }
                            }
                        },
                        None => return
                    }
                }
            },
            None => return
        }
    }

    fn parse_identifier_or_keyword_or_opcode(&mut self) -> Token {
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

        if self.is_opcode(&parsed_identifier) {
            return Token {
                ttype: TokenType::Opcode(parsed_identifier),
                line: self.line,
                start_column: start_column,
                end_column: end_column,
                source_file: self.source_file.to_string(),
            };
        } else {
            return Token {
                ttype: TokenType::Identifier(parsed_identifier),
                line: self.line,
                start_column: start_column,
                end_column: end_column,
                source_file: self.source_file.to_string(),
            };
        }
    }

    fn parse_hex_number(&mut self) -> Token {
        let start_column = self.column;

        // Eat $
        self.consume();

        let mut parsed_number = String::new();

        loop {
            match self.peek() {
                None => break,
                Some(&current_char) => {
                    if is_ascii_hex_digit(current_char) {
                        parsed_number.push(self.consume().unwrap())
                    } else {
                        break;
                    }
                }
            }
        }

        let end_column = self.column;

        let result_number = match u32::from_str_radix(&parsed_number, 16) {
            Ok(result) => result,
            Err(_) => 0
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
            argument_size: argument_size
        };

        self.new_token(TokenType::NumberLiteral(number_literal), start_column, end_column)
    }

    fn parse_binary_number(&mut self) -> Token {
        let start_column = self.column;

        // Eat %
        self.consume();

        let mut parsed_number = String::new();

        loop {
            match self.peek() {
                None => break,
                Some(&current_char) => {
                    if is_ascii_binary_digit(current_char) {
                        parsed_number.push(self.consume().unwrap())
                    } else {
                        break;
                    }
                }
            }
        }

        let end_column = self.column;

        let result_number = match u32::from_str_radix(&parsed_number, 2) {
            Ok(result) => result,
            Err(_) => 0
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
            argument_size: argument_size
        };

        self.new_token(TokenType::NumberLiteral(number_literal), start_column, end_column)
    }

    fn parse_number(&mut self) -> Token {
        let start_column = self.column;
        let mut parsed_number = String::new();

        parsed_number.push(self.consume().unwrap());

        loop {
            match self.peek() {
                None => break,
                Some(&current_char) => {
                    if is_ascii_numeric(current_char) {
                        parsed_number.push(self.consume().unwrap())
                    } else {
                        break;
                    }
                }
            }
        }

        let end_column = self.column;

        let result_number = match u32::from_str_radix(&parsed_number, 10) {
            Ok(result) => result,
            Err(_) => 0
        };

        let argument_size = if result_number > 16777215 {
            ArgumentSize::Word32
        } else if result_number > u16::max_value() as u32 {
            ArgumentSize::Word24
        } else if result_number > u8::max_value() as u32 {
            ArgumentSize::Word16
        } else {
            ArgumentSize::Word8
        };

        let number_literal = NumberLiteral {
            number: result_number,
            argument_size: argument_size
        };

        self.new_token(TokenType::NumberLiteral(number_literal), start_column, end_column)
    }

    fn is_opcode(&self, identifier: &str) -> bool {
        for instruction in self.system.instructions.iter() {
            if instruction.name == identifier {
                return true;
            }
        }

        return false;
    }

    fn do_end_of_line(&mut self) {
        self.line += 1;
        self.column = 0;

        self.consume();
        self.start_line = self.it.clone();
    }

    fn token_invalid(&mut self) -> Token {
        let invalid_char = match self.consume() {
            Some(result) => result,
            None => ' ',
        };

        let start_column = self.column - 1;
        let end_column = self.column;

        self.new_token(TokenType::Invalid(invalid_char), start_column, end_column)
    }

    fn token_eof(&mut self) -> Token {
        let start_column = self.column;
        let end_column = self.column;

        self.new_token(TokenType::EndOfFile, start_column, end_column)
    }

    fn new_token(&mut self, ttype: TokenType, start_column: u32, end_column: u32) -> Token {
        Token {
            ttype: ttype,
            line: self.line,
            start_column: start_column,
            end_column: end_column,
            source_file: self.source_file.to_string(),
        }
    }

    fn peek(&mut self) -> Option<&char> {
        match self.it.peek() {
            None => None,
            Some(result) => Some(result),
        }
    }
    
    fn peek_lookahead(&mut self, lookahead: usize) -> Option<char> {
        let mut skip_it = self.it.clone().skip(lookahead);

        match skip_it.next() {
            Some(result) => Some(result),
            None => None
        }
    }

    fn consume(&mut self) -> Option<char> {
        match self.it.next() {
            None => None,
            Some(result) => {
                self.column += 1;
                return Some(result);
            }
        }
    }
}
