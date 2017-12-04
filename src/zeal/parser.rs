use zeal::lexer::*;
use zeal::system_definition::*;

#[derive(Clone)]
pub enum ParseArgument {
    NumberLiteral(NumberLiteral),
    Register(String)
}

#[derive(Clone)]
pub enum Statement {
    ImpliedInstruction(&'static InstructionInfo),
    SingleArgumentInstruction(&'static InstructionInfo, ParseArgument),
    IndexedInstruction(&'static InstructionInfo, ParseArgument),
}

#[derive(Clone)]
pub enum ParseExpression {
    ImpliedInstruction(String),
    ImmediateInstruction(String, ParseArgument),
    SingleArgumentInstruction(String, ParseArgument),
    IndexedInstruction(String, ParseArgument, ParseArgument),
    Statement(Statement)
}

#[derive(Clone)]
pub struct ParseNode<'a> {
    pub start_token: Token<'a>,
    pub expression: ParseExpression
}

#[derive(PartialEq)]
pub enum ErrorSeverity {
    Error,
    Warning
}

pub struct ErrorMessage<'a> {
    pub message: String,
    pub token: Token<'a>,
    pub severity: ErrorSeverity
}

pub struct Parser<'a> {
    lexers: Vec<Lexer<'a>>,
    pub error_messages: Vec<ErrorMessage<'a>>
}

enum ParseResult<T> {
    None,
    Done,
    Error,
    Some(T)
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut lexers = Vec::new();
        lexers.push(lexer);

        Parser {
            lexers: lexers,
            error_messages: Vec::new()
        }
    }

    pub fn has_errors(&self) -> bool {
        return !self.error_messages.is_empty();
    }

    pub fn parse_tree(&mut self) -> Vec<ParseNode<'a>> {
        let mut parsed_tree = Vec::new();

        loop {
            match self.parse() {
                ParseResult::Some(node) => parsed_tree.push(node),
                ParseResult::None => continue,
                ParseResult::Error => continue,
                ParseResult::Done => break,
            }
        }

        return parsed_tree;
    }

    fn parse(&mut self) -> ParseResult<ParseNode<'a>> {
        // root : (cpuInstruction)* ;
        let token = self.get_next_token();
        match token.ttype {
            TokenType::EndOfFile => return ParseResult::Done,
            TokenType::Opcode(ref opcode_name) => self.parse_cpu_instruction(&token, opcode_name),
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

    fn parse_cpu_instruction(&mut self, opcode_token: &Token<'a>, opcode_name: &str) -> ParseResult<ParseNode<'a>> {
        // cpuInstruction : OPCODE #Implied
        //    | OPCODE '#' argument #Immediate
        //    | OPCODE argument #SingleArgument
        //    | OPCODE argument,register #Indexed
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
                if is_immediate {
                    return ParseResult::Some(ParseNode {
                        start_token: opcode_token.clone(),
                        expression: ParseExpression::ImmediateInstruction(opcode_name.to_string(), result)
                    });
                } else {
                    let comma = self.lookahead();
                    if comma.ttype == TokenType::Comma {
                        self.get_next_token();

                        let second_lookahead = self.lookahead();

                        match second_lookahead.ttype {
                            TokenType::Register(_) => {
                                let second_argument = self.parse_argument();
                                match second_argument {
                                    ParseResult::Some(second_result) => {
                                        return ParseResult::Some(ParseNode {
                                            start_token: opcode_token.clone(),
                                            expression: ParseExpression::IndexedInstruction(opcode_name.to_string(), result, second_result)
                                        });
                                    },
                                    ParseResult::None => {
                                        self.add_error_message(&format!("expected register as second argument."), second_lookahead);
                                        return ParseResult::Error
                                    },
                                    ParseResult::Error => {
                                        return ParseResult::Error
                                    },
                                    ParseResult::Done => {
                                        return ParseResult::Done
                                    }
                                }
                            },
                            _ => {
                                self.get_next_token();
                                self.add_error_message(&format!("expected register as second argument."), second_lookahead);
                                return ParseResult::Error
                            }
                        }
                    }

                    return ParseResult::Some(ParseNode {
                        start_token: opcode_token.clone(),
                        expression: ParseExpression::SingleArgumentInstruction(opcode_name.to_string(), result)
                    });
                }
            },
            ParseResult::None => {
                return ParseResult::Some(ParseNode {
                    start_token: opcode_token.clone(),
                    expression: ParseExpression::ImpliedInstruction(opcode_name.to_string())
                });
            },
            ParseResult::Error => {
                return ParseResult::Error
            },
            ParseResult::Done => {
                return ParseResult::Done
            }
        };
    }

    fn parse_argument(&mut self) -> ParseResult<ParseArgument> {
        // argument : NUMBER_LITERAL
        //          | REGISTER
        //          ;

        let lookahead = self.lookahead();
        match lookahead.ttype {
            TokenType::NumberLiteral(number_literal) => {
                self.get_next_token(); // Eat tokenNumberLiteral
                ParseResult::Some(ParseArgument::NumberLiteral(number_literal))
            },
            TokenType::Register(register_name) => {
                self.get_next_token(); // Eat register token
                ParseResult::Some(ParseArgument::Register(register_name))
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
                self.add_error_message(&format!("A number literal or register was expected here."), lookahead);
                ParseResult::Error
            }
        }
    }

    fn lookahead(&mut self) -> Token<'a> {
        self.lexer().unwrap().lookahead()
    }

    fn get_next_token(&mut self) -> Token<'a> {
        self.lexer().unwrap().get_next_token()
    }

    fn lexer(&mut self) -> Option<&mut Lexer<'a>> {
        self.lexers.last_mut()
    }

    fn add_error_message(&mut self, error_message: &str, offending_token: Token<'a>) {
        let new_message = ErrorMessage {
            message: error_message.to_owned(),
            token: offending_token,
            severity: ErrorSeverity::Error
        };

        self.error_messages.push(new_message);
    }

    fn add_invalid_token_message(&mut self, invalid_token: char, token: Token<'a>) {
        self.add_error_message(&format!("Invalid token '{}' found.", invalid_token), token);
    }
}
