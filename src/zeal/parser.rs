use zeal::lexer::*;
use zeal::system_definition::*;

#[derive(Clone)]
pub enum ParseArgument {
    NumberLiteral(NumberLiteral),
    Register(String),
    Identifier(String)
}

#[derive(Clone)]
pub enum FinalInstruction {
    ImpliedInstruction(&'static InstructionInfo),
    SingleArgumentInstruction(&'static InstructionInfo, ParseArgument),
    TwoArgumentInstruction(&'static InstructionInfo, ParseArgument, ParseArgument),
}

#[derive(Clone)]
pub enum ParseExpression {
    ImpliedInstruction(String),
    ImmediateInstruction(String, ParseArgument),
    SingleArgumentInstruction(String, ParseArgument),
    IndexedInstruction(String, ParseArgument, ParseArgument),
    IndirectInstruction(String, ParseArgument),
    IndirectLongInstruction(String, ParseArgument),
    IndexedIndirectInstruction(String, ParseArgument, ParseArgument),
    IndirectIndexedInstruction(String, ParseArgument, ParseArgument),
    IndirectIndexedLongInstruction(String, ParseArgument, ParseArgument),
    BlockMoveInstruction(String, ParseArgument, ParseArgument),
    StackRelativeIndirectIndexedInstruction(String, ParseArgument, ParseArgument, ParseArgument),
    FinalInstruction(FinalInstruction),
    Label(String),
    OriginStatement(NumberLiteral),
}

#[derive(Clone)]
pub struct ParseNode<'a> {
    pub start_token: Token<'a>,
    pub expression: ParseExpression,
}

#[derive(PartialEq)]
pub enum ErrorSeverity {
    Error,
    Warning,
}

pub struct ErrorMessage<'a> {
    pub message: String,
    pub token: Token<'a>,
    pub severity: ErrorSeverity,
}

pub struct Parser<'a> {
    lexers: Vec<Lexer<'a>>,
    pub error_messages: Vec<ErrorMessage<'a>>,
}

enum ParseResult<T> {
    None,
    Done,
    Error,
    Some(T),
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut lexers = Vec::new();
        lexers.push(lexer);

        Parser {
            lexers: lexers,
            error_messages: Vec::new(),
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

    // root : (cpuInstruction | label | origin_statement)*;
    fn parse(&mut self) -> ParseResult<ParseNode<'a>> {
        let token = self.get_next_token();
        match token.ttype {
            TokenType::EndOfFile => return ParseResult::Done,
            TokenType::Opcode(ref opcode_name) => self.parse_cpu_instruction(&token, opcode_name),
            TokenType::Identifier(ref label_name) => {
                self.parse_label(&token, label_name)
            }
            TokenType::KeywordOrigin => {
                self.parse_origin_statement(&token)
            }
            TokenType::Invalid(invalid_token) => {
                self.add_invalid_token_message(invalid_token, token);
                return ParseResult::Error;
            }
            _ => {
                self.add_error_message("unexpected token found.", token);
                return ParseResult::Error;
            }
        }
    }

    // cpuInstruction : OPCODE #Implied
    //    | OPCODE '#' argument #Immediate
    //    | OPCODE argument #SingleArgument
    //    | OPCODE argument,register #Indexed
    //    | OPCODE (argument) #Indirect
    //    | OPCODE [argument] #IndirectLong
    //    | OPCODE (argument,register) #IndexedIndirect
    //    | OPCODE (argument),register #IndirectIndexed
    //    | OPCODE [argument],register #IndirectIndexedLong
    //    | OPCODE (argument,register),register #StackRelativeIndirectIndexed
    //    ;
    fn parse_cpu_instruction(
        &mut self,
        opcode_token: &Token<'a>,
        opcode_name: &str,
    ) -> ParseResult<ParseNode<'a>> {
        let lookahead = self.lookahead();

        if lookahead.ttype == TokenType::Immediate {
            return self.parse_immediate(opcode_token, opcode_name);
        } else if lookahead.ttype == TokenType::LeftParen {
            return self.parse_indirect(opcode_token, opcode_name);
        } else if lookahead.ttype == TokenType::LeftBracket {
            return self.parse_indirect_long(opcode_token, opcode_name);
        } else {
            let argument = self.parse_argument();

            match argument {
                ParseResult::Some(result) => {
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
                                            expression: ParseExpression::IndexedInstruction(
                                                opcode_name.to_string(),
                                                result,
                                                second_result,
                                            ),
                                        });
                                    }
                                    ParseResult::None => {
                                        self.add_error_message(
                                            &format!(
                                                "expected register or register as second argument."
                                            ),
                                            opcode_token.clone(),
                                        );
                                        return ParseResult::Error;
                                    }
                                    ParseResult::Error => return ParseResult::Error,
                                    ParseResult::Done => return ParseResult::Done,
                                }
                            }
                            TokenType::NumberLiteral(_) => {
                                let second_argument = self.parse_argument();
                                match second_argument {
                                    ParseResult::Some(second_result) => {
                                        return ParseResult::Some(ParseNode {
                                            start_token: opcode_token.clone(),
                                            expression: ParseExpression::BlockMoveInstruction(
                                                opcode_name.to_string(),
                                                result,
                                                second_result,
                                            ),
                                        });
                                    }
                                    ParseResult::None => {
                                        self.add_error_message(
                                            &format!(
                                                "expected number or register as second argument."
                                            ),
                                            opcode_token.clone(),
                                        );
                                        return ParseResult::Error;
                                    }
                                    ParseResult::Error => return ParseResult::Error,
                                    ParseResult::Done => return ParseResult::Done,
                                }
                            }
                            _ => {
                                self.get_next_token();
                                self.add_error_message(
                                    &format!("expected number or register as second argument."),
                                    opcode_token.clone(),
                                );
                                return ParseResult::Error;
                            }
                        }
                    }

                    return ParseResult::Some(ParseNode {
                        start_token: opcode_token.clone(),
                        expression: ParseExpression::SingleArgumentInstruction(
                            opcode_name.to_string(),
                            result,
                        ),
                    });
                }
                ParseResult::None => {
                    return ParseResult::Some(ParseNode {
                        start_token: opcode_token.clone(),
                        expression: ParseExpression::ImpliedInstruction(opcode_name.to_string()),
                    });
                }
                ParseResult::Error => {
                    return ParseResult::Error;
                }
                ParseResult::Done => {
                    return ParseResult::Done;
                }
            };
        }
    }

    fn parse_immediate(
        &mut self,
        opcode_token: &Token<'a>,
        opcode_name: &str,
    ) -> ParseResult<ParseNode<'a>> {
        self.get_next_token();

        let argument = self.parse_argument();

        match argument {
            ParseResult::Some(result) => {
                return ParseResult::Some(ParseNode {
                    start_token: opcode_token.clone(),
                    expression: ParseExpression::ImmediateInstruction(
                        opcode_name.to_string(),
                        result,
                    ),
                });
            }
            // Found an opcode
            ParseResult::None => {
                self.add_error_message(
                    &format!("number expected as argument."),
                    opcode_token.clone(),
                );
                return ParseResult::Error;
            }
            ParseResult::Error => {
                return ParseResult::Error;
            }
            ParseResult::Done => {
                return ParseResult::Done;
            }
        };
    }

    fn parse_indirect(
        &mut self,
        opcode_token: &Token<'a>,
        opcode_name: &str,
    ) -> ParseResult<ParseNode<'a>> {
        let left_paren = self.get_next_token(); // Eat left parenthesis

        let argument = self.parse_argument();

        match argument {
            ParseResult::Some(result) => {
                let lookahead = self.lookahead();

                if lookahead.ttype == TokenType::RightParen {
                    self.get_next_token(); // Eat right parenthesis

                    let second_lookahead = self.lookahead();
                    if second_lookahead.ttype == TokenType::Comma {
                        self.get_next_token(); // Eat comma

                        let second_argument = self.parse_argument();

                        match second_argument {
                            ParseResult::Some(second_result) => {
                                return ParseResult::Some(ParseNode {
                                    start_token: opcode_token.clone(),
                                    expression: ParseExpression::IndirectIndexedInstruction(
                                        opcode_name.to_string(),
                                        result,
                                        second_result,
                                    ),
                                });
                            }
                            ParseResult::None => {
                                self.add_error_message(
                                    &format!("register expected as argument."),
                                    opcode_token.clone(),
                                );
                                return ParseResult::Error;
                            }
                            ParseResult::Done => return ParseResult::Done,
                            ParseResult::Error => return ParseResult::Error,
                        }
                    } else {
                        return ParseResult::Some(ParseNode {
                            start_token: opcode_token.clone(),
                            expression: ParseExpression::IndirectInstruction(
                                opcode_name.to_string(),
                                result,
                            ),
                        });
                    }
                } else if lookahead.ttype == TokenType::Comma {
                    self.get_next_token(); // Eat comma

                    let second_argument = self.parse_argument();

                    match second_argument {
                        ParseResult::Some(second_result) => {
                            let second_lookahead = self.lookahead();
                            if second_lookahead.ttype == TokenType::RightParen {
                                self.get_next_token(); // Eat right parenthesis

                                let third_lookahead = self.lookahead();
                                if third_lookahead.ttype == TokenType::Comma {
                                    self.get_next_token(); // Eat comma

                                    let third_argument = self.parse_argument();

                                    match third_argument {
                                        ParseResult::Some(third_result) => {
                                            return ParseResult::Some(ParseNode {
                                                start_token: opcode_token.clone(),
                                                expression: ParseExpression::StackRelativeIndirectIndexedInstruction(
                                                    opcode_name.to_string(),
                                                    result,
                                                    second_result,
                                                    third_result
                                                ),
                                            });
                                        }
                                        ParseResult::None => {
                                            self.add_error_message(
                                                &format!("register expected as argument."),
                                                opcode_token.clone(),
                                            );
                                            return ParseResult::Error;
                                        }
                                        ParseResult::Done => return ParseResult::Done,
                                        ParseResult::Error => return ParseResult::Error
                                    }
                                } else {
                                    return ParseResult::Some(ParseNode {
                                        start_token: opcode_token.clone(),
                                        expression: ParseExpression::IndexedIndirectInstruction(
                                            opcode_name.to_string(),
                                            result,
                                            second_result,
                                        ),
                                    });
                                }
                            } else {
                                self.add_error_message(
                                    &format!("no closing parenthesis found."),
                                    left_paren,
                                );
                                return ParseResult::Error;
                            }
                        }
                        ParseResult::None => {
                            self.add_error_message(
                                &format!("register expected as argument."),
                                opcode_token.clone(),
                            );
                            return ParseResult::Error;
                        }
                        ParseResult::Done => return ParseResult::Done,
                        ParseResult::Error => return ParseResult::Error,
                    }
                } else {
                    self.add_error_message(&format!("no closing parenthesis found."), left_paren);
                    return ParseResult::Error;
                }
            }
            // Found an opcode
            ParseResult::None => {
                self.add_error_message(
                    &format!("number expected as argument."),
                    opcode_token.clone(),
                );
                return ParseResult::Error;
            }
            ParseResult::Error => {
                return ParseResult::Error;
            }
            ParseResult::Done => {
                return ParseResult::Done;
            }
        };
    }

    fn parse_indirect_long(
        &mut self,
        opcode_token: &Token<'a>,
        opcode_name: &str,
    ) -> ParseResult<ParseNode<'a>> {
        let left_bracket = self.get_next_token(); // Eat left bracket

        let argument = self.parse_argument();

        match argument {
            ParseResult::Some(result) => {
                let lookahead = self.lookahead();

                if lookahead.ttype == TokenType::RightBracket {
                    self.get_next_token(); // Eat right bracket

                    let second_lookahead = self.lookahead();
                    if second_lookahead.ttype == TokenType::Comma {
                        self.get_next_token(); // Eat comma

                        let second_argument = self.parse_argument();

                        match second_argument {
                            ParseResult::Some(second_result) => {
                                return ParseResult::Some(ParseNode {
                                    start_token: opcode_token.clone(),
                                    expression: ParseExpression::IndirectIndexedLongInstruction(
                                        opcode_name.to_string(),
                                        result,
                                        second_result,
                                    ),
                                });
                            }
                            ParseResult::None => {
                                self.add_error_message(
                                    &format!("register expected as argument."),
                                    opcode_token.clone(),
                                );
                                return ParseResult::Error;
                            }
                            ParseResult::Done => return ParseResult::Done,
                            ParseResult::Error => return ParseResult::Error,
                        }
                    } else {
                        return ParseResult::Some(ParseNode {
                            start_token: opcode_token.clone(),
                            expression: ParseExpression::IndirectLongInstruction(
                                opcode_name.to_string(),
                                result,
                            ),
                        });
                    }
                } else {
                    self.add_error_message(&format!("no closing bracket found."), left_bracket);
                    return ParseResult::Error;
                }
            }
            // Found an opcode
            ParseResult::None => {
                self.add_error_message(
                    &format!("number expected as argument."),
                    opcode_token.clone(),
                );
                return ParseResult::Error;
            }
            ParseResult::Error => {
                return ParseResult::Error;
            }
            ParseResult::Done => {
                return ParseResult::Done;
            }
        };
    }

    // argument : NUMBER_LITERAL
    //          | REGISTER
    //          | IDENTIFIER
    //          ;
    fn parse_argument(&mut self) -> ParseResult<ParseArgument> {
        let lookahead = self.lookahead();
        match lookahead.ttype {
            TokenType::NumberLiteral(number_literal) => {
                self.get_next_token(); // Eat tokenNumberLiteral
                ParseResult::Some(ParseArgument::NumberLiteral(number_literal))
            }
            TokenType::Register(register_name) => {
                self.get_next_token(); // Eat register token
                ParseResult::Some(ParseArgument::Register(register_name))
            }
            TokenType::Identifier(identifier) => {
                self.get_next_token(); // Eat identifier token
                ParseResult::Some(ParseArgument::Identifier(identifier))
            }
            TokenType::Opcode(_) => ParseResult::None,
            TokenType::Invalid(invalid_token) => {
                self.get_next_token(); // Eat token
                self.add_invalid_token_message(invalid_token, lookahead);
                ParseResult::Error
            }
            TokenType::EndOfFile => ParseResult::Done,
            _ => {
                self.get_next_token(); // Eat token
                self.add_error_message(
                    &format!("A number literal or register was expected here."),
                    lookahead,
                );
                ParseResult::Error
            }
        }
    }

    // label : IDENTIFIER ':'
    fn parse_label(&mut self, label_token: &Token<'a>, label_name: &str) -> ParseResult<ParseNode<'a>> {
        let lookahead = self.lookahead();

        if lookahead.ttype == TokenType::Colon {
            self.get_next_token(); // Eat colon
            return ParseResult::Some(ParseNode {
                    start_token: label_token.clone(),
                    expression: ParseExpression::Label(label_name.to_string()),
                });
        } else {
            self.add_error_message(&"Expected a colon after this identifier.", label_token.clone());
            return ParseResult::Error;
        }
    }

    // origin_statement: 'origin' NUMBER_LITERAL
    fn parse_origin_statement(&mut self, origin_token: &Token<'a>) -> ParseResult<ParseNode<'a>> {
        let lookahead = self.lookahead();

        match lookahead.ttype {
            TokenType::NumberLiteral(number) => {
                self.get_next_token(); // Eat literal
                return ParseResult::Some(ParseNode {
                    start_token: origin_token.clone(),
                    expression: ParseExpression::OriginStatement(number),
                });
            }
            TokenType::Invalid(invalid_token) => {
                self.get_next_token(); // Eat token
                self.add_invalid_token_message(invalid_token, lookahead);
                ParseResult::Error
            }
            TokenType::EndOfFile => ParseResult::Done,
            _ => {
                self.add_error_message(&"Expected a number literal after origin keyword.", origin_token.clone());
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
            severity: ErrorSeverity::Error,
        };

        self.error_messages.push(new_message);
    }

    fn add_invalid_token_message(&mut self, invalid_token: char, token: Token<'a>) {
        self.add_error_message(&format!("Invalid token '{}' found.", invalid_token), token);
    }
}
