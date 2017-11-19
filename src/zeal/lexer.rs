use std::str::Chars;
use std::iter::Peekable;

pub enum TokenType {
    Invalid(char),
    Identifier(String),
    EndOfFile,
}

pub struct Token {
    pub ttype: TokenType,
    pub line: u32,
    pub start_column: u32,
    pub end_column: u32,
    pub source_file: String
}

pub struct Lexer<'a> {
    it: Peekable<Chars<'a>>,
    source_file: String,
    line: u32,
    column: u32,
}

impl<'a>  Lexer<'a> {
    pub fn new(file_content: &str) -> Lexer {
        Lexer {
            line: 0,
            column: 0,
            it: file_content.chars().peekable(),
            source_file: String::from("")
        }
    }

    pub fn get_next_token(&mut self) -> Token {
        self.eat_whitespaces();

        match self.peek() {
            None => self.token_eof(),
            Some(&current_char) => self.parse_token(current_char)
        }
    }

    fn parse_token(&mut self, current_char: char) -> Token {
        match current_char {
                'a'...'z' | 'A'...'Z' | '_' => {
                        return self.parse_identifier_or_keyword();
                    },
                _ => return self.token_invalid()
            }
    }

    fn eat_whitespaces(&mut self) {
        while let Some(&current_char) = self.peek() {
            if current_char == '\n' {
                self.line += 1;
                self.column = 0;
            }

            if !current_char.is_whitespace() {
                break;
            }

            self.consume();
        }
    }

    fn parse_identifier_or_keyword(&mut self) -> Token {
        let start_column = self.column;
        let mut parsed_identifier = String::new();

        parsed_identifier.push(self.consume().unwrap());

        loop {
            match self.peek() {
                None => break,
                Some(&current_char) => {
                    if self.is_ascii_alphanumeric(current_char) || current_char == '_' {
                        parsed_identifier.push(self.consume().unwrap())
                    } else {
                        break;
                    }
                }
            }
        }

        let end_column = self.column;

        return Token {
            ttype: TokenType::Identifier(parsed_identifier),
            line: self.line,
            start_column: start_column,
            end_column: end_column,
            source_file: self.source_file.to_string()
        };
    }

     fn is_ascii_alphanumeric(&self, current_char: char) -> bool {
        (current_char >= '0' && current_char <= '9')
        || (current_char >= 'A' && current_char <= 'Z')
        || (current_char >= 'a' && current_char <= 'z')
    }

    fn token_invalid(&mut self) -> Token {
        let invalid_char = match self.consume() {
            Some(result) => result,
            None => ' '
        };

        Token {
            ttype: TokenType::Invalid(invalid_char),
            line: self.line,
            start_column: self.column,
            end_column: self.column,
            source_file: self.source_file.to_string()
        }
    }

    fn token_eof(&mut self) -> Token {
        Token {
            ttype: TokenType::EndOfFile,
            line: self.line,
            start_column: self.column,
            end_column: self.column,
            source_file: self.source_file.to_string()
        }
    }

    fn peek(&mut self) -> Option<&char> {
        match self.it.peek() {
            None => None,
            Some(result) => Some(result)
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
