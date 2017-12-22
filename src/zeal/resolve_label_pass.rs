use zeal::lexer::*;
use zeal::parser::*;
use zeal::system_definition::*;
use zeal::pass::TreePass;
use zeal::symbol_table::*;

pub struct ResolveLabelPass<'a> {
    system: &'static SystemDefinition,
    pub error_messages: Vec<ErrorMessage<'a>>,
}

impl<'a> ResolveLabelPass<'a> {
    pub fn new(system: &'static SystemDefinition) -> Self {
        ResolveLabelPass {
            system: system,
            error_messages: Vec::new()
        }
    }

    fn add_error_message(&mut self, error_message: &str, offending_token: Token<'a>) {
        let new_message = ErrorMessage {
            message: error_message.to_owned(),
            token: offending_token,
            severity: ErrorSeverity::Error
        };

        self.error_messages.push(new_message);
    }

    fn find_instruction_argument_size(&self, opcode_name: &str, possible_addressings: &[AddressingMode]) -> Option<ArgumentSize> {
        for instruction in self.system.instructions.iter() {
            if instruction.name == opcode_name {
                for addressing_mode in possible_addressings.iter() {
                    if &instruction.addressing == addressing_mode {
                        for argument in instruction.arguments {
                            match argument {
                                &InstructionArgument::Number(argument_size) => {
                                    return Some(argument_size);
                                }
                                &InstructionArgument::Numbers(ref sizes) => {
                                    if sizes.len() > 0 {
                                        return Some(sizes[0]);
                                    }
                                }
                                _ => {}
                            };
                        }
                    }
                }
            }
        }

        return None
    }

    fn is_branching_instruction(&self, opcode_name: &str) -> bool {
        for instruction in self.system.instructions.iter() {
            if instruction.name == opcode_name {
                if instruction.addressing == AddressingMode::Relative {
                    return true;
                }
            }
        }

        return false;
    }
}

impl<'a> TreePass<'a> for ResolveLabelPass<'a> {
    fn has_errors(&self) -> bool {
        return !self.error_messages.is_empty()
    }

    fn get_error_messages(&self) -> &Vec<ErrorMessage<'a>> {
        &self.error_messages
    }

    fn do_pass(&mut self, parse_tree: Vec<ParseNode<'a>>, symbol_table: &mut SymbolTable) -> Vec<ParseNode<'a>> {
        let mut new_tree:Vec<ParseNode<'a>> = Vec::new();

        let mut current_address:u32 = 0;

        for node in parse_tree.iter() {
            match node.expression {
                ParseExpression::ImpliedInstruction(_) => {
                    new_tree.push(node.clone());
                    current_address += 1;
                }
                ParseExpression::ImmediateInstruction(ref opcode_name, ref argument) => {
                    current_address += 1;

                    match argument {
                        &ParseArgument::Identifier(ref identifier) => {
                            if symbol_table.has_label(identifier) {

                                let argument_size = match self.find_instruction_argument_size(opcode_name, &[AddressingMode::Immediate]) {
                                    Some(size) => size,
                                    None =>  self.system.label_size
                                };

                                let number = NumberLiteral {
                                    number: symbol_table.address_for(identifier),
                                    argument_size: argument_size
                                };

                                current_address += argument_size_to_byte_size(argument_size);

                                new_tree.push(ParseNode {
                                    start_token: node.start_token.clone(),
                                    expression: ParseExpression::ImmediateInstruction(opcode_name.to_owned(), ParseArgument::NumberLiteral(number))
                                });
                            } else {
                                self.add_error_message(&format!("Label '{}' not found.", identifier), node.start_token.clone());
                                new_tree.push(node.clone());
                            }
                        }
                        &ParseArgument::NumberLiteral(ref number) => {
                            current_address += argument_size_to_byte_size(number.argument_size);
                            new_tree.push(node.clone());
                        }
                        _ => {
                            new_tree.push(node.clone());
                        }
                    }
                }
                ParseExpression::SingleArgumentInstruction(ref opcode_name, ref argument) => {
                    current_address += 1;

                    match argument {
                        &ParseArgument::Identifier(ref identifier) => {
                            if symbol_table.has_label(identifier) {
                                let argument_size = match self.find_instruction_argument_size(opcode_name, &[AddressingMode::SingleArgument, AddressingMode::Relative]) {
                                    Some(size) => size,
                                    None =>  self.system.label_size
                                };

                                let mut address = 0;

                                if self.is_branching_instruction(opcode_name) {
                                    match argument_size {
                                        ArgumentSize::Word8 => {
                                            let temp_address:i64 = (symbol_table.address_for(identifier) as i64) - ((current_address + argument_size_to_byte_size(argument_size)) as i64);
                                            if temp_address > (i8::max_value() as i64) || temp_address < (i8::min_value() as i64)
                                            {
                                                println!("address: {}, current_address: {}", symbol_table.address_for(identifier), current_address);
                                                self.add_error_message(&format!("Branch label '{0}' is too far away. Consider reducing the distance of the label.", identifier), node.start_token.clone());
                                            }
                                            else
                                            {
                                                address = (temp_address as u32) & 0xFF;
                                            }
                                        }
                                        ArgumentSize::Word16 => {
                                            let temp_address:i64 = (symbol_table.address_for(identifier) as i64) - ((current_address + argument_size_to_byte_size(argument_size)) as i64);
                                            if temp_address > (i16::max_value() as i64) || temp_address < (i16::min_value() as i64)
                                            {
                                                self.add_error_message(&format!("Branch label '{0}' is too far away. Consider reducing the distance of the label.", identifier), node.start_token.clone());
                                            }
                                            else
                                            {
                                                address = (temp_address as u32) & 0xFFFF;
                                            }
                                        }
                                        _ => {}
                                    };
                                } else {
                                    address = symbol_table.address_for(identifier);
                                }

                                let number = NumberLiteral {
                                    number: address,
                                    argument_size: argument_size
                                };

                                current_address += argument_size_to_byte_size(argument_size);

                                new_tree.push(ParseNode {
                                    start_token: node.start_token.clone(),
                                    expression: ParseExpression::SingleArgumentInstruction(opcode_name.to_owned(), ParseArgument::NumberLiteral(number))
                                });
                            } else {
                                self.add_error_message(&format!("Label '{}' not found.", identifier), node.start_token.clone());
                                new_tree.push(node.clone());
                            }
                        }
                        &ParseArgument::NumberLiteral(ref number) => {
                            current_address += argument_size_to_byte_size(number.argument_size);
                            new_tree.push(node.clone());
                        }
                        _ => {
                            new_tree.push(node.clone());
                        }
                    }
                }
                ParseExpression::IndexedInstruction(ref opcode_name, ref argument1, ref argument2) => {
                    current_address += 1;

                    match argument1 {
                        &ParseArgument::Identifier(ref identifier) => {
                            if symbol_table.has_label(identifier) {
                                let argument_size = match self.find_instruction_argument_size(opcode_name, &[AddressingMode::Indexed]) {
                                    Some(size) => size,
                                    None =>  self.system.label_size
                                };

                                let number = NumberLiteral {
                                    number: symbol_table.address_for(identifier),
                                    argument_size: argument_size
                                };

                                current_address += argument_size_to_byte_size(argument_size);

                                new_tree.push(ParseNode {
                                    start_token: node.start_token.clone(),
                                    expression: ParseExpression::IndexedInstruction(opcode_name.to_owned(), ParseArgument::NumberLiteral(number), argument2.clone())
                                });
                            } else {
                                self.add_error_message(&format!("Label '{}' not found.", identifier), node.start_token.clone());
                                new_tree.push(node.clone());
                            }
                        }
                        &ParseArgument::NumberLiteral(ref number) => {
                            current_address += argument_size_to_byte_size(number.argument_size);
                             new_tree.push(node.clone());
                        }
                        _ => {
                             new_tree.push(node.clone());
                        }
                    };
                }
                ParseExpression::IndirectInstruction(ref opcode_name, ref argument) => {
                    new_tree.push(node.clone());
                    current_address += 1;

                    match argument {
                        &ParseArgument::Identifier(ref identifier) => {
                            if symbol_table.has_label(identifier) {

                                let argument_size = match self.find_instruction_argument_size(opcode_name, &[AddressingMode::Indirect]) {
                                    Some(size) => size,
                                    None =>  self.system.label_size
                                };

                                let number = NumberLiteral {
                                    number: symbol_table.address_for(identifier),
                                    argument_size: argument_size
                                };

                                current_address += argument_size_to_byte_size(argument_size);

                                new_tree.push(ParseNode {
                                    start_token: node.start_token.clone(),
                                    expression: ParseExpression::IndirectInstruction(opcode_name.to_owned(), ParseArgument::NumberLiteral(number))
                                });
                            } else {
                                self.add_error_message(&format!("Label '{}' not found.", identifier), node.start_token.clone());
                                new_tree.push(node.clone());
                            }
                        }
                        &ParseArgument::NumberLiteral(ref number) => {
                            current_address += argument_size_to_byte_size(number.argument_size);
                            new_tree.push(node.clone());
                        }
                        _ => {
                            new_tree.push(node.clone());
                        }
                    };
                }
                ParseExpression::IndirectLongInstruction(ref opcode_name, ref argument) => {
                    new_tree.push(node.clone());

                    match argument {
                        &ParseArgument::Identifier(ref identifier) => {
                            if symbol_table.has_label(identifier) {

                                let argument_size = match self.find_instruction_argument_size(opcode_name, &[AddressingMode::IndirectLong]) {
                                    Some(size) => size,
                                    None =>  self.system.label_size
                                };

                                let number = NumberLiteral {
                                    number: symbol_table.address_for(identifier),
                                    argument_size: argument_size
                                };

                                current_address += argument_size_to_byte_size(argument_size);

                                new_tree.push(ParseNode {
                                    start_token: node.start_token.clone(),
                                    expression: ParseExpression::IndirectLongInstruction(opcode_name.to_owned(), ParseArgument::NumberLiteral(number))
                                });
                            } else {
                                self.add_error_message(&format!("Label '{}' not found.", identifier), node.start_token.clone());
                                new_tree.push(node.clone());
                            }
                        }
                        &ParseArgument::NumberLiteral(ref number) => {
                            current_address += argument_size_to_byte_size(number.argument_size);
                            new_tree.push(node.clone());
                        }
                        _ => {
                            new_tree.push(node.clone());
                        }
                    }
                }
                ParseExpression::IndexedIndirectInstruction(ref opcode_name, ref argument1, ref argument2) => {
                    current_address += 1;

                    match argument1 {
                        &ParseArgument::Identifier(ref identifier) => {
                            if symbol_table.has_label(identifier) {
                                let argument_size = match self.find_instruction_argument_size(opcode_name, &[AddressingMode::IndexedIndirect]) {
                                    Some(size) => size,
                                    None =>  self.system.label_size
                                };

                                let number = NumberLiteral {
                                    number: symbol_table.address_for(identifier),
                                    argument_size: argument_size
                                };

                                current_address += argument_size_to_byte_size(argument_size);

                                new_tree.push(ParseNode {
                                    start_token: node.start_token.clone(),
                                    expression: ParseExpression::IndexedIndirectInstruction(opcode_name.to_owned(), ParseArgument::NumberLiteral(number), argument2.clone())
                                });
                            } else {
                                self.add_error_message(&format!("Label '{}' not found.", identifier), node.start_token.clone());
                                new_tree.push(node.clone());
                            }
                        }
                        &ParseArgument::NumberLiteral(ref number) => {
                            current_address += argument_size_to_byte_size(number.argument_size);
                             new_tree.push(node.clone());
                        }
                        _ => {
                             new_tree.push(node.clone());
                        }
                    };
                }
                ParseExpression::IndirectIndexedInstruction(ref opcode_name, ref argument1, ref argument2) => {
                    current_address += 1;

                    match argument1 {
                        &ParseArgument::Identifier(ref identifier) => {
                            if symbol_table.has_label(identifier) {
                                let argument_size = match self.find_instruction_argument_size(opcode_name, &[AddressingMode::IndirectIndexed]) {
                                    Some(size) => size,
                                    None =>  self.system.label_size
                                };

                                let number = NumberLiteral {
                                    number: symbol_table.address_for(identifier),
                                    argument_size: argument_size
                                };

                                current_address += argument_size_to_byte_size(argument_size);

                                new_tree.push(ParseNode {
                                    start_token: node.start_token.clone(),
                                    expression: ParseExpression::IndirectIndexedInstruction(opcode_name.to_owned(), ParseArgument::NumberLiteral(number), argument2.clone())
                                });
                            } else {
                                self.add_error_message(&format!("Label '{}' not found.", identifier), node.start_token.clone());
                                new_tree.push(node.clone());
                            }
                        }
                        &ParseArgument::NumberLiteral(ref number) => {
                            current_address += argument_size_to_byte_size(number.argument_size);
                             new_tree.push(node.clone());
                        }
                        _ => {
                             new_tree.push(node.clone());
                        }
                    };
                }
                ParseExpression::IndirectIndexedLongInstruction(ref opcode_name, ref argument1, ref argument2) => {
                    current_address += 1;

                    match argument1 {
                        &ParseArgument::Identifier(ref identifier) => {
                            if symbol_table.has_label(identifier) {
                                let argument_size = match self.find_instruction_argument_size(opcode_name, &[AddressingMode::IndirectIndexedLong]) {
                                    Some(size) => size,
                                    None =>  self.system.label_size
                                };

                                let number = NumberLiteral {
                                    number: symbol_table.address_for(identifier),
                                    argument_size: argument_size
                                };

                                current_address += argument_size_to_byte_size(argument_size);

                                new_tree.push(ParseNode {
                                    start_token: node.start_token.clone(),
                                    expression: ParseExpression::IndirectIndexedLongInstruction(opcode_name.to_owned(), ParseArgument::NumberLiteral(number), argument2.clone())
                                });
                            } else {
                                self.add_error_message(&format!("Label '{}' not found.", identifier), node.start_token.clone());
                                new_tree.push(node.clone());
                            }
                        }
                        &ParseArgument::NumberLiteral(ref number) => {
                            current_address += argument_size_to_byte_size(number.argument_size);
                             new_tree.push(node.clone());
                        }
                        _ => {
                             new_tree.push(node.clone());
                        }
                    };
                }
                ParseExpression::BlockMoveInstruction(_, ref argument1, ref argument2) => {
                    new_tree.push(node.clone());
                    current_address += 1;

                    match argument1 {
                        &ParseArgument::NumberLiteral(ref number) => {
                            current_address += argument_size_to_byte_size(number.argument_size);
                        }
                        _ => {}
                    };

                    match argument2 {
                        &ParseArgument::NumberLiteral(ref number) => {
                            current_address += argument_size_to_byte_size(number.argument_size);
                        }
                        _ => {}
                    };
                }
                ParseExpression::StackRelativeIndirectIndexedInstruction(ref opcode_name, ref argument1, ref argument2, ref argument3) => {
                    current_address += 1;

                    match argument1 {
                        &ParseArgument::Identifier(ref identifier) => {
                            if symbol_table.has_label(identifier) {
                                let argument_size = match self.find_instruction_argument_size(opcode_name, &[AddressingMode::StackRelativeIndirectIndexed]) {
                                    Some(size) => size,
                                    None =>  self.system.label_size
                                };

                                let number = NumberLiteral {
                                    number: symbol_table.address_for(identifier),
                                    argument_size: argument_size
                                };

                                current_address += argument_size_to_byte_size(argument_size);

                                new_tree.push(ParseNode {
                                    start_token: node.start_token.clone(),
                                    expression: ParseExpression::StackRelativeIndirectIndexedInstruction(opcode_name.to_owned(), ParseArgument::NumberLiteral(number), argument2.clone(), argument3.clone())
                                });
                            } else {
                                self.add_error_message(&format!("Label '{}' not found.", identifier), node.start_token.clone());
                                new_tree.push(node.clone());
                            }
                        }
                        &ParseArgument::NumberLiteral(ref number) => {
                            current_address += argument_size_to_byte_size(number.argument_size);
                             new_tree.push(node.clone());
                        }
                        _ => {
                             new_tree.push(node.clone());
                        }
                    };
                }
                ParseExpression::OriginStatement(ref number) => {
                    current_address = number.number;
                    new_tree.push(node.clone());
                }
                ParseExpression::IncBinStatement(_, file_size) => {
                    current_address += file_size as u32;
                    new_tree.push(node.clone());
                }
                _ => {
                    new_tree.push(node.clone());
                }
            }
        }

        return new_tree;
    }
}
