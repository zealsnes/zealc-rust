use zeal::parser::*;
use zeal::system_definition::*;
use zeal::pass::TreePass;
use zeal::symbol_table::*;

pub struct CollectLabelPass<'a> {
    system: &'static SystemDefinition,
    pub error_messages: Vec<ErrorMessage<'a>>,
}

impl<'a> CollectLabelPass<'a> {
    pub fn new(system: &'static SystemDefinition) -> Self {
        CollectLabelPass {
            system: system,
            error_messages: Vec::new(),
        }
    }

    // fn add_error_message(&mut self, error_message: &str, offending_token: Token<'a>) {
    //     let new_message = ErrorMessage {
    //         message: error_message.to_owned(),
    //         token: offending_token,
    //         severity: ErrorSeverity::Error
    //     };

    //     self.error_messages.push(new_message);
    // }

    fn find_instruction_argument_size(
        &self,
        opcode_name: &str,
        possible_addressings: &[AddressingMode],
    ) -> Option<ArgumentSize> {
        for instruction in self.system.instructions.iter() {
            if instruction.name == opcode_name {
                for addressing_mode in possible_addressings.iter() {
                    if &instruction.addressing == addressing_mode {
                        for argument in instruction.arguments {
                            match argument {
                                &InstructionArgument::Number(argument_size) => {
                                    return Some(argument_size);
                                }
                                &InstructionArgument::Numbers(ref sizes) => if sizes.len() > 0 {
                                    return Some(sizes[0]);
                                },
                                _ => {}
                            };
                        }
                    }
                }
            }
        }

        return None;
    }
}

impl<'a> TreePass<'a> for CollectLabelPass<'a> {
    fn has_errors(&self) -> bool {
        return !self.error_messages.is_empty();
    }

    fn get_error_messages(&self) -> &Vec<ErrorMessage<'a>> {
        &self.error_messages
    }

    fn do_pass(
        &mut self,
        parse_tree: Vec<ParseNode<'a>>,
        symbol_table: &mut SymbolTable,
    ) -> Vec<ParseNode<'a>> {
        let mut new_tree: Vec<ParseNode<'a>> = Vec::new();

        let mut current_address: u32 = 0;

        for node in parse_tree.iter() {
            match node.expression {
                ParseExpression::ImpliedInstruction(_) => {
                    new_tree.push(node.clone());
                    current_address += 1;
                }
                ParseExpression::ImmediateInstruction(_, ref argument) => {
                    new_tree.push(node.clone());
                    current_address += 1;

                    match argument {
                        &ParseArgument::NumberLiteral(ref number) => {
                            current_address += argument_size_to_byte_size(number.argument_size);
                        }
                        &ParseArgument::Identifier(_) => {
                            current_address += argument_size_to_byte_size(self.system.label_size);
                        }
                        _ => {}
                    }
                }
                ParseExpression::SingleArgumentInstruction(ref opcode_name, ref argument) => {
                    new_tree.push(node.clone());
                    current_address += 1;

                    match argument {
                        &ParseArgument::NumberLiteral(ref number) => {
                            current_address += argument_size_to_byte_size(number.argument_size);
                        }
                        &ParseArgument::Identifier(_) => {
                            match self.find_instruction_argument_size(
                                opcode_name,
                                &[AddressingMode::SingleArgument, AddressingMode::Relative],
                            ) {
                                Some(size) => current_address += argument_size_to_byte_size(size),
                                None => {
                                    current_address +=
                                        argument_size_to_byte_size(self.system.label_size);
                                }
                            };
                        }
                        _ => {}
                    }
                }
                ParseExpression::IndexedInstruction(
                    ref opcode_name,
                    ref argument1,
                    ref argument2,
                ) => {
                    new_tree.push(node.clone());
                    current_address += 1;

                    match argument1 {
                        &ParseArgument::NumberLiteral(ref number) => {
                            current_address += argument_size_to_byte_size(number.argument_size);
                        }
                        &ParseArgument::Identifier(_) => {
                            match self.find_instruction_argument_size(
                                opcode_name,
                                &[AddressingMode::Indexed],
                            ) {
                                Some(size) => current_address += argument_size_to_byte_size(size),
                                None => {
                                    current_address +=
                                        argument_size_to_byte_size(self.system.label_size);
                                }
                            };
                        }
                        _ => {}
                    };

                    match argument2 {
                        &ParseArgument::NumberLiteral(ref number) => {
                            current_address += argument_size_to_byte_size(number.argument_size);
                        }
                        &ParseArgument::Identifier(_) => {
                            match self.find_instruction_argument_size(
                                opcode_name,
                                &[AddressingMode::Indexed],
                            ) {
                                Some(size) => current_address += argument_size_to_byte_size(size),
                                None => {
                                    current_address +=
                                        argument_size_to_byte_size(self.system.label_size);
                                }
                            };
                        }
                        _ => {}
                    };
                }
                ParseExpression::IndirectInstruction(ref opcode_name, ref argument) => {
                    new_tree.push(node.clone());
                    current_address += 1;

                    match argument {
                        &ParseArgument::NumberLiteral(ref number) => {
                            current_address += argument_size_to_byte_size(number.argument_size);
                        }
                        &ParseArgument::Identifier(_) => {
                            match self.find_instruction_argument_size(
                                opcode_name,
                                &[AddressingMode::Indirect],
                            ) {
                                Some(size) => current_address += argument_size_to_byte_size(size),
                                None => {
                                    current_address +=
                                        argument_size_to_byte_size(self.system.label_size);
                                }
                            };
                        }
                        _ => {}
                    }
                }
                ParseExpression::IndirectLongInstruction(ref opcode_name, ref argument) => {
                    new_tree.push(node.clone());
                    current_address += 1;

                    match argument {
                        &ParseArgument::NumberLiteral(ref number) => {
                            current_address += argument_size_to_byte_size(number.argument_size);
                        }
                        &ParseArgument::Identifier(_) => {
                            match self.find_instruction_argument_size(
                                opcode_name,
                                &[AddressingMode::Indirect],
                            ) {
                                Some(size) => current_address += argument_size_to_byte_size(size),
                                None => {
                                    current_address +=
                                        argument_size_to_byte_size(self.system.label_size);
                                }
                            };
                        }
                        _ => {}
                    }
                }
                ParseExpression::IndexedIndirectInstruction(
                    ref opcode_name,
                    ref argument1,
                    ref argument2,
                ) => {
                    new_tree.push(node.clone());
                    current_address += 1;

                    match argument1 {
                        &ParseArgument::NumberLiteral(ref number) => {
                            current_address += argument_size_to_byte_size(number.argument_size);
                        }
                        &ParseArgument::Identifier(_) => {
                            match self.find_instruction_argument_size(
                                opcode_name,
                                &[AddressingMode::IndexedIndirect],
                            ) {
                                Some(size) => current_address += argument_size_to_byte_size(size),
                                None => {
                                    current_address +=
                                        argument_size_to_byte_size(self.system.label_size);
                                }
                            };
                        }
                        _ => {}
                    };

                    match argument2 {
                        &ParseArgument::NumberLiteral(ref number) => {
                            current_address += argument_size_to_byte_size(number.argument_size);
                        }
                        &ParseArgument::Identifier(_) => {
                            match self.find_instruction_argument_size(
                                opcode_name,
                                &[AddressingMode::IndexedIndirect],
                            ) {
                                Some(size) => current_address += argument_size_to_byte_size(size),
                                None => {
                                    current_address +=
                                        argument_size_to_byte_size(self.system.label_size);
                                }
                            };
                        }
                        _ => {}
                    };
                }
                ParseExpression::IndirectIndexedInstruction(
                    ref opcode_name,
                    ref argument1,
                    ref argument2,
                ) => {
                    new_tree.push(node.clone());
                    current_address += 1;

                    match argument1 {
                        &ParseArgument::NumberLiteral(ref number) => {
                            current_address += argument_size_to_byte_size(number.argument_size);
                        }
                        &ParseArgument::Identifier(_) => {
                            match self.find_instruction_argument_size(
                                opcode_name,
                                &[AddressingMode::IndirectIndexed],
                            ) {
                                Some(size) => current_address += argument_size_to_byte_size(size),
                                None => {
                                    current_address +=
                                        argument_size_to_byte_size(self.system.label_size);
                                }
                            };
                        }
                        _ => {}
                    };

                    match argument2 {
                        &ParseArgument::NumberLiteral(ref number) => {
                            current_address += argument_size_to_byte_size(number.argument_size);
                        }
                        &ParseArgument::Identifier(_) => {
                            match self.find_instruction_argument_size(
                                opcode_name,
                                &[AddressingMode::IndirectIndexed],
                            ) {
                                Some(size) => current_address += argument_size_to_byte_size(size),
                                None => {
                                    current_address +=
                                        argument_size_to_byte_size(self.system.label_size);
                                }
                            };
                        }
                        _ => {}
                    };
                }
                ParseExpression::IndirectIndexedLongInstruction(
                    ref opcode_name,
                    ref argument1,
                    ref argument2,
                ) => {
                    new_tree.push(node.clone());
                    current_address += 1;

                    match argument1 {
                        &ParseArgument::NumberLiteral(ref number) => {
                            current_address += argument_size_to_byte_size(number.argument_size);
                        }
                        &ParseArgument::Identifier(_) => {
                            match self.find_instruction_argument_size(
                                opcode_name,
                                &[AddressingMode::IndirectIndexedLong],
                            ) {
                                Some(size) => current_address += argument_size_to_byte_size(size),
                                None => {
                                    current_address +=
                                        argument_size_to_byte_size(self.system.label_size);
                                }
                            };
                        }
                        _ => {}
                    };

                    match argument2 {
                        &ParseArgument::NumberLiteral(ref number) => {
                            current_address += argument_size_to_byte_size(number.argument_size);
                        }
                        &ParseArgument::Identifier(_) => {
                            match self.find_instruction_argument_size(
                                opcode_name,
                                &[AddressingMode::IndirectIndexedLong],
                            ) {
                                Some(size) => current_address += argument_size_to_byte_size(size),
                                None => {
                                    current_address +=
                                        argument_size_to_byte_size(self.system.label_size);
                                }
                            };
                        }
                        _ => {}
                    };
                }
                ParseExpression::BlockMoveInstruction(
                    ref opcode_name,
                    ref argument1,
                    ref argument2,
                ) => {
                    new_tree.push(node.clone());
                    current_address += 1;

                    match argument1 {
                        &ParseArgument::NumberLiteral(ref number) => {
                            current_address += argument_size_to_byte_size(number.argument_size);
                        }
                        &ParseArgument::Identifier(_) => {
                            match self.find_instruction_argument_size(
                                opcode_name,
                                &[AddressingMode::BlockMove],
                            ) {
                                Some(size) => current_address += argument_size_to_byte_size(size),
                                None => {
                                    current_address +=
                                        argument_size_to_byte_size(self.system.label_size);
                                }
                            };
                        }
                        _ => {}
                    };

                    match argument2 {
                        &ParseArgument::NumberLiteral(ref number) => {
                            current_address += argument_size_to_byte_size(number.argument_size);
                        }
                        &ParseArgument::Identifier(_) => {
                            match self.find_instruction_argument_size(
                                opcode_name,
                                &[AddressingMode::BlockMove],
                            ) {
                                Some(size) => current_address += argument_size_to_byte_size(size),
                                None => {
                                    current_address +=
                                        argument_size_to_byte_size(self.system.label_size);
                                }
                            };
                        }
                        _ => {}
                    };
                }
                ParseExpression::StackRelativeIndirectIndexedInstruction(
                    ref opcode_name,
                    ref argument1,
                    ref argument2,
                    ref argument3,
                ) => {
                    new_tree.push(node.clone());
                    current_address += 1;

                    match argument1 {
                        &ParseArgument::NumberLiteral(ref number) => {
                            current_address += argument_size_to_byte_size(number.argument_size);
                        }
                        &ParseArgument::Identifier(_) => {
                            match self.find_instruction_argument_size(
                                opcode_name,
                                &[AddressingMode::StackRelativeIndirectIndexed],
                            ) {
                                Some(size) => current_address += argument_size_to_byte_size(size),
                                None => {
                                    current_address +=
                                        argument_size_to_byte_size(self.system.label_size);
                                }
                            };
                        }
                        _ => {}
                    };

                    match argument2 {
                        &ParseArgument::NumberLiteral(ref number) => {
                            current_address += argument_size_to_byte_size(number.argument_size);
                        }
                        &ParseArgument::Identifier(_) => {
                            match self.find_instruction_argument_size(
                                opcode_name,
                                &[AddressingMode::BlockMove],
                            ) {
                                Some(size) => current_address += argument_size_to_byte_size(size),
                                None => {
                                    current_address +=
                                        argument_size_to_byte_size(self.system.label_size);
                                }
                            };
                        }
                        _ => {}
                    };

                    match argument3 {
                        &ParseArgument::NumberLiteral(ref number) => {
                            current_address += argument_size_to_byte_size(number.argument_size);
                        }
                        &ParseArgument::Identifier(_) => {
                            match self.find_instruction_argument_size(
                                opcode_name,
                                &[AddressingMode::StackRelativeIndirectIndexed],
                            ) {
                                Some(size) => current_address += argument_size_to_byte_size(size),
                                None => {
                                    current_address +=
                                        argument_size_to_byte_size(self.system.label_size);
                                }
                            };
                        }
                        _ => {}
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
                ParseExpression::Label(ref label_name) => {
                    symbol_table.add_or_update_label(label_name, current_address);
                }
                _ => {
                    new_tree.push(node.clone());
                }
            }
        }

        return new_tree;
    }
}
