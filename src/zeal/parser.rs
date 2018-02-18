use std::fs::{metadata};
use std::path::{Path, PathBuf};
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
pub enum SnesMap {
    LoRom,
    HiRom,
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
    SnesMapStatement(SnesMap),
    IncBinStatement(String, u64),
}

#[derive(Clone)]
pub struct ParseNode {
    pub start_token: Token,
    pub expression: ParseExpression,
}

#[derive(PartialEq)]
pub enum ErrorSeverity {
    Error,
    Warning,
}

pub struct ErrorMessage {
    pub message: String,
    pub token: Token,
    pub severity: ErrorSeverity,
}

pub struct Parser {
    system: &'static SystemDefinition,
    lexers: Vec<Lexer>,
    current_lexer: i32,
    pub error_messages: Vec<ErrorMessage>,
}

enum ParseResult<T> {
    None,
    Done,
    Error,
    Some(T),
}

impl Parser {
    pub fn new(system: &'static SystemDefinition) -> Self {
        Parser {
            system: system,
            lexers: Vec::new(),
            error_messages: Vec::new(),
            current_lexer: -1,
        }
    }

    pub fn set_current_input_file(&mut self, filename: &str) {
        for index in 0..self.lexers.len() {
            if self.lexers[index].source_file == filename {
                self.current_lexer = index as i32;
                self.lexers[index].reset();
                return;
            }
        }

        self.lexers.push(Lexer::from_file(self.system, filename));
        self.current_lexer = (self.lexers.len() - 1) as i32;
    }

    pub fn has_errors(&self) -> bool {
        return !self.error_messages.is_empty();
    }

    pub fn parse_tree(&mut self) -> Vec<ParseNode> {
        let mut parsed_tree = Vec::new();

        loop {
            match self.parse() {
                ParseResult::Some(node) => parsed_tree.push(node),
                ParseResult::None => continue,
                ParseResult::Error => continue,
                ParseResult::Done => {
                    self.current_lexer -= 1;
                    if self.current_lexer < 0 {
                        break
                    }
                }
            }
        }

        return parsed_tree;
    }

    // root : (cpuInstruction | label | origin_statement | snesmap_statement | incbin_statement | include_statement)*;
    fn parse(&mut self) -> ParseResult<ParseNode> {
        let token = self.get_next_token();
        match token.ttype {
            TokenType::EndOfFile => return ParseResult::Done,
            TokenType::Opcode(ref opcode_name) => self.parse_cpu_instruction(&token, opcode_name),
            TokenType::Identifier(ref label_name) => {
                self.parse_label(&token, label_name)
            }
            TokenType::KeywordInclude => {
                self.parse_include(&token)
            }
            TokenType::KeywordIncbin => {
                self.parse_incbin(&token)
            }
            TokenType::KeywordOrigin => {
                self.parse_origin_statement(&token)
            }
            TokenType::KeywordSnesMap => {
                self.parse_snesmap_statement(&token)
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
        opcode_token: &Token,
        opcode_name: &str,
    ) -> ParseResult<ParseNode> {
        let lookahead = self.lookahead(1);

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
                    let comma = self.lookahead(1);
                    if comma.ttype == TokenType::Comma {
                        self.get_next_token();

                        let second_lookahead = self.lookahead(1);

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
                ParseResult::None | ParseResult::Done => {
                    return ParseResult::Some(ParseNode {
                        start_token: opcode_token.clone(),
                        expression: ParseExpression::ImpliedInstruction(opcode_name.to_string()),
                    });
                }
                ParseResult::Error => {
                    return ParseResult::Error;
                }
            };
        }
    }

    fn parse_immediate(
        &mut self,
        opcode_token: &Token,
        opcode_name: &str,
    ) -> ParseResult<ParseNode> {
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
        opcode_token: &Token,
        opcode_name: &str,
    ) -> ParseResult<ParseNode> {
        let left_paren = self.get_next_token(); // Eat left parenthesis

        let argument = self.parse_argument();

        match argument {
            ParseResult::Some(result) => {
                let lookahead = self.lookahead(1);

                if lookahead.ttype == TokenType::RightParen {
                    self.get_next_token(); // Eat right parenthesis

                    let second_lookahead = self.lookahead(1);
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
                            let second_lookahead = self.lookahead(1);
                            if second_lookahead.ttype == TokenType::RightParen {
                                self.get_next_token(); // Eat right parenthesis

                                let third_lookahead = self.lookahead(1);
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
        opcode_token: &Token,
        opcode_name: &str,
    ) -> ParseResult<ParseNode> {
        let left_bracket = self.get_next_token(); // Eat left bracket

        let argument = self.parse_argument();

        match argument {
            ParseResult::Some(result) => {
                let lookahead = self.lookahead(1);

                if lookahead.ttype == TokenType::RightBracket {
                    self.get_next_token(); // Eat right bracket

                    let second_lookahead = self.lookahead(1);
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
        let lookahead = self.lookahead(1);
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
                let second_lookahead = self.lookahead(2);
                if second_lookahead.ttype == TokenType::Colon {
                    return ParseResult::None
                } else {
                    self.get_next_token(); // Eat identifier token
                    ParseResult::Some(ParseArgument::Identifier(identifier))
                }
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
    fn parse_label(&mut self, label_token: &Token, label_name: &str) -> ParseResult<ParseNode> {
        let lookahead = self.lookahead(1);

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
    fn parse_origin_statement(&mut self, origin_token: &Token) -> ParseResult<ParseNode> {
        let lookahead = self.lookahead(1);

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

    // snesmap_statement: 'snesmap' ('lorom'|'hirom')
    fn parse_snesmap_statement(&mut self, origin_token: &Token) -> ParseResult<ParseNode> {
        let lookahead = self.lookahead(1);

        match lookahead.ttype {
            TokenType::Identifier(identifier) => {
                self.get_next_token(); // Eat literal
                match self.identifier_to_snesmap(&identifier) {
                    Some(snes_map) => {
                        return ParseResult::Some(ParseNode {
                            start_token: origin_token.clone(),
                            expression: ParseExpression::SnesMapStatement(snes_map),
                        });
                    }
                    None => {
                        self.add_error_message(&"Expected lorom or hirom as argument to snesmap.", origin_token.clone());
                        ParseResult::Error
                    }
                }
            }
            TokenType::Invalid(invalid_token) => {
                self.get_next_token(); // Eat token
                self.add_invalid_token_message(invalid_token, lookahead);
                ParseResult::Error
            }
            TokenType::EndOfFile => ParseResult::Done,
            _ => {
                self.add_error_message(&"Expected lorom or hirom as argument to snesmap.", origin_token.clone());
                ParseResult::Error
            }
        }
    }

    // include_statement : 'include' STRING_LITERAL
    fn parse_include(&mut self, origin_token: &Token) -> ParseResult<ParseNode> {
        let lookahead = self.lookahead(1);

        match lookahead.ttype {
            TokenType::StringLiteral(filename) => {
                let source_filename = self.lexer().unwrap().source_file.to_string();
                let source_file_path = Path::new(&source_filename);
                let mut include_path = PathBuf::new();
                include_path.push(source_file_path.parent().unwrap());
                include_path.push(&filename);

                match metadata(&include_path) {
                    Ok(_) => {
                        self.get_next_token(); // eat string literal
                        self.set_current_input_file(include_path.to_str().unwrap()); // Make the current lexer the included file

                        ParseResult::None
                    }
                    _ => {
                        self.get_next_token(); // eat string literal
                        self.add_error_message(&format!("Couldn't open file '{}' for include statement", filename), origin_token.clone());
                        ParseResult::Error
                    }
                }
            }
            TokenType::Invalid(invalid_token) => {
                self.get_next_token(); // Eat token
                self.add_invalid_token_message(invalid_token, lookahead);
                ParseResult::Error
            }
            TokenType::EndOfFile => ParseResult::Done,
            _ => {
                self.add_error_message(&"Expected a string literal as argument to incbin", origin_token.clone());
                ParseResult::Error
            }
        }
    }

    // incbin_statement : 'incbin' STRING_LITERAL
    fn parse_incbin(&mut self, origin_token: &Token) -> ParseResult<ParseNode> {
        let lookahead = self.lookahead(1);

        match lookahead.ttype {
            TokenType::StringLiteral(filename) => {
                let source_filename = self.lexer().unwrap().source_file.to_string();
                let source_file_path = Path::new(&source_filename);
                let mut incbin_path = PathBuf::new();
                incbin_path.push(source_file_path.parent().unwrap());
                incbin_path.push(&filename);

                match metadata(&incbin_path) {
                    Ok(file_metadata) => {
                        self.get_next_token(); // eat string literal
                        let file_size = file_metadata.len();
                        return ParseResult::Some(ParseNode {
                            start_token: origin_token.clone(),
                            expression: ParseExpression::IncBinStatement(incbin_path.to_str().unwrap().to_string(), file_size),
                        });
                    }
                    _ => {
                        self.get_next_token(); // eat string literal
                        self.add_error_message(&format!("Couldn't open file '{}' for incbin statement", filename), origin_token.clone());
                        ParseResult::Error
                    }
                }
            }
            TokenType::Invalid(invalid_token) => {
                self.get_next_token(); // Eat token
                self.add_invalid_token_message(invalid_token, lookahead);
                ParseResult::Error
            }
            TokenType::EndOfFile => ParseResult::Done,
            _ => {
                self.add_error_message(&"Expected a string literal as argument to incbin", origin_token.clone());
                ParseResult::Error
            }
        }
    }

    fn identifier_to_snesmap(&self, identifier: &str) -> Option<SnesMap> {
        if identifier == "lorom" {
            Some(SnesMap::LoRom)
        } else if identifier == "hirom" {
            Some(SnesMap::HiRom)
        } else {
            None
        }
    }

    fn lookahead(&mut self, times: u32) -> Token {
        self.lexer().unwrap().lookahead(times)
    }

    fn get_next_token(&mut self) -> Token {
        self.lexer().unwrap().get_next_token()
    }

    fn lexer(&mut self) -> Option<&mut Lexer> {
        if self.current_lexer >= 0 && self.current_lexer < (self.lexers.len() as i32) {
            Some(&mut self.lexers[self.current_lexer as usize])
        } else {
            None
        }
    }

    fn add_error_message(&mut self, error_message: &str, offending_token: Token) {
        let new_message = ErrorMessage {
            message: error_message.to_owned(),
            token: offending_token,
            severity: ErrorSeverity::Error,
        };

        self.error_messages.push(new_message);
    }

    fn add_invalid_token_message(&mut self, invalid_token: char, token: Token) {
        self.add_error_message(&format!("Invalid token '{}' found.", invalid_token), token);
    }
}
