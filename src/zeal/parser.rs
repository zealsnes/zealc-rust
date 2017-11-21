use zeal::lexer::*;
use zeal::system_definition::*;

pub enum ArgumentExpression {
    NumberLiteralExpression(NumberLiteral),
}

pub enum Expression {
    ImpliedInstruction(&'static InstructionInfo),
    SingleArgumentInstruction(&'static InstructionInfo, ArgumentExpression),
}

pub struct Parser<'a> {
    lexers: Vec<Lexer<'a>>,
    system: &'static SystemDefinition,
}

impl<'a> Parser<'a> {
    pub fn new(system: &'static SystemDefinition, lexer: Lexer<'a>) -> Self {
        let mut lexers = Vec::new();
        lexers.push(lexer);

        Parser {
            lexers: lexers,
            system: system,
        }
    }

    pub fn parse_tree(&mut self) -> Vec<Expression> {
        let mut expressions = Vec::new();

        loop {
            match self.parse() {
                Some(expression) => expressions.push(expression),
                None => break,
            }
        }

        return expressions;
    }

    fn parse(&mut self) -> Option<Expression> {
        // root : (cpuInstruction)* ;
        let token = self.get_next_token();
        match token.ttype {
            TokenType::Invalid(_) => return None,
            TokenType::EndOfFile => return None,
            TokenType::Opcode(opcode_name) => self.parse_cpu_instruction(opcode_name),
            _ => return None,
        }
    }

    fn parse_cpu_instruction(&mut self, opcode_name: String) -> Option<Expression> {
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
            Some(result) => {
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
                            Some(instruction) => return Some(Expression::SingleArgumentInstruction(instruction, result)),
                            None => return None
                        }
                    }
                }
            },
            None => {
                let possible_instruction = self.find_suitable_instruction(&opcode_name, &[AddressingMode::Implied]);
                match possible_instruction {
                    Some(instruction) => return Some(Expression::ImpliedInstruction(instruction)),
                    None => return None
                }
            }
        };
    }

    fn parse_argument(&mut self) -> Option<ArgumentExpression> {
        // argument : NUMBER_LITERAL ;
        let lookahead = self.lookahead();
        match lookahead.ttype {
            TokenType::NumberLiteral(number_literal) => {
                self.get_next_token(); // Eat token
                Some(ArgumentExpression::NumberLiteralExpression(number_literal))
            },
            _ => {
                None
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
}
