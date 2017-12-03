use std::str::Chars;
use std::iter::Peekable;
use zeal::lexer::*;
use zeal::system_definition::*;

pub enum ArgumentExpression {
    NumberLiteralExpression(NumberLiteral),
}

pub enum Expression {
    ImpliedInstruction(&'static InstructionInfo),
    SingleArgumentInstruction(&'static InstructionInfo, ArgumentExpression),
}

#[derive(PartialEq)]
pub enum ErrorSeverity {
    Error,
    Warning
}

pub struct ErrorMessage<'a> {
    pub message: String,
    pub token: Token,
    pub context_start: Peekable<Chars<'a>>,
    pub severity: ErrorSeverity
}

pub struct Parser<'a> {
    lexers: Vec<Lexer<'a>>,
    system: &'static SystemDefinition,
    pub error_messages: Vec<ErrorMessage<'a>>
}

enum ParseResult<T> {
    None,
    Done,
    Error,
    Some(T)
}

impl<'a> Parser<'a> {
    pub fn new(system: &'static SystemDefinition, lexer: Lexer<'a>) -> Self {
        let mut lexers = Vec::new();
        lexers.push(lexer);

        Parser {
            lexers: lexers,
            system: system,
            error_messages: Vec::new()
        }
    }

    pub fn has_errors(&self) -> bool {
        return !self.error_messages.is_empty();
    }

    pub fn parse_tree(&mut self) -> Vec<Expression> {
        let mut expressions = Vec::new();

        loop {
            match self.parse() {
                ParseResult::Some(expression) => expressions.push(expression),
                ParseResult::None => continue,
                ParseResult::Error => continue,
                ParseResult::Done => break,
            }
        }

        return expressions;
    }

    fn parse(&mut self) -> ParseResult<Expression> {
        // root : (cpuInstruction)* ;
        let token = self.get_next_token();
        match token.ttype {
            TokenType::EndOfFile => return ParseResult::Done,
            TokenType::Opcode(opcode_name) => self.parse_cpu_instruction(opcode_name),
            TokenType::Invalid(invalid_token) => {
                self.add_invalid_token_message(invalid_token, token);
                return ParseResult::Error;
            },
             _ => {
                self.add_error_message("unexpected token found.", token);
                return ParseResult::Error
            }
        }
    }

    fn parse_cpu_instruction(&mut self, opcode_name: String) -> ParseResult<Expression> {
        // cpuInstruction : OPCODE #Implied
        //    | OPCODE '#' argument #Immediate
        //    | OPCODE argument #SingleArgument
        //    ;
        let lookahead = self.lookahead();

        let mut is_immediate = false;
        if lookahead.ttype == TokenType::Immediate {
            self.get_next_token();
            is_immediate = true;
        }

        let argument = self.parse_argument();

        match argument {
            ParseResult::Some(result) => {
                match result {
                    ArgumentExpression::NumberLiteralExpression(number_literal) => {
                        let possible_instruction = if is_immediate {
                            self.find_suitable_instruction(&opcode_name, &[AddressingMode::Immediate])
                        } else if number_literal.argument_size == ArgumentSize::Word24 {
                            self.find_suitable_instruction(&opcode_name, &[AddressingMode::AbsoluteLong])
                        } else if number_literal.argument_size == ArgumentSize::Word16 {
                            self.find_suitable_instruction(&opcode_name, &[AddressingMode::Absolute, AddressingMode::RelativeLong])
                        } else {
                            self.find_suitable_instruction(&opcode_name, &[AddressingMode::Direct, AddressingMode::Relative])
                        };

                        match possible_instruction {
                            Some(instruction) => return ParseResult::Some(Expression::SingleArgumentInstruction(instruction, result)),
                            None => return ParseResult::Error
                        }
                    }
                }
            },
            ParseResult::None => {
                let possible_instruction = self.find_suitable_instruction(&opcode_name, &[AddressingMode::Implied]);
                match possible_instruction {
                    Some(instruction) => return ParseResult::Some(Expression::ImpliedInstruction(instruction)),
                    None => return ParseResult::Error
                }
            },
            ParseResult::Error => {
                return ParseResult::Error
            },
            ParseResult::Done => {
                return ParseResult::Done
            }
        };
    }

    fn parse_argument(&mut self) -> ParseResult<ArgumentExpression> {
        // argument : NUMBER_LITERAL ;
        let lookahead = self.lookahead();
        match lookahead.ttype {
            TokenType::NumberLiteral(number_literal) => {
                self.get_next_token(); // Eat token
                ParseResult::Some(ArgumentExpression::NumberLiteralExpression(number_literal))
            },
            TokenType::Opcode(_) => {
                ParseResult::None
            },
            TokenType::Invalid(invalid_token) => {
                self.get_next_token(); // Eat token
                self.add_invalid_token_message(invalid_token, lookahead);
                ParseResult::Error
            },
            TokenType::EndOfFile => {
                ParseResult::Done
            },
            _ => {
                self.get_next_token(); // Eat token
                self.add_error_message(&format!("A number litteral was expected here."), lookahead);
                ParseResult::Error
            }
        }
    }

    fn find_suitable_instruction(&mut self, opcode_name: &str, possible_addressings: &[AddressingMode]) -> Option<&'static InstructionInfo> {
        for instruction in self.system.instructions.iter() {
            if instruction.name == opcode_name {
                for addressing_mode in possible_addressings.iter() {
                    if &instruction.addressing == addressing_mode {
                        return Some(instruction)
                    }
                }
            }
        }

        return None
    }

    fn lookahead(&mut self) -> Token {
        self.lexer().unwrap().lookahead()
    }

    fn get_next_token(&mut self) -> Token {
        self.lexer().unwrap().get_next_token()
    }

    fn lexer(&mut self) -> Option<&mut Lexer<'a>> {
        self.lexers.last_mut()
    }

    fn add_error_message(&mut self, error_message: &str, offending_token: Token) {
        let new_message = ErrorMessage {
            message: error_message.to_owned(),
            token: offending_token,
            context_start: self.lexer().unwrap().start_line.clone(),
            severity: ErrorSeverity::Error
        };

        self.error_messages.push(new_message);
    }

    fn add_invalid_token_message(&mut self, invalid_token: char, token: Token) {
        self.add_error_message(&format!("Invalid token '{}' found.", invalid_token), token);
    }
}
