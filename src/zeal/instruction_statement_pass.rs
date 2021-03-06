use zeal::lexer::Token;
use zeal::parser::*;
use zeal::system_definition::*;
use zeal::pass::TreePass;
use zeal::symbol_table::SymbolTable;

pub struct InstructionToStatementPass {
    system: &'static SystemDefinition,
    pub error_messages: Vec<ErrorMessage>,
}

impl InstructionToStatementPass {
    pub fn new(system: &'static SystemDefinition) -> Self {
        InstructionToStatementPass {
            system: system,
            error_messages: Vec::new(),
        }
    }

    fn find_suitable_instruction(
        &mut self,
        opcode_name: &str,
        possible_addressings: &[AddressingMode],
        possible_arguments: &[InstructionArgument],
    ) -> Option<&'static InstructionInfo> {
        for instruction in self.system.instructions.iter() {
            if instruction.name == opcode_name {
                for addressing_mode in possible_addressings.iter() {
                    if &instruction.addressing == addressing_mode {
                        let mut same_arguments = true;
                        let argument_size = instruction.arguments.len();
                        let possible_size = possible_arguments.len();

                        if argument_size != possible_size {
                            same_arguments = false;
                        }
                        if same_arguments {
                            for i in 0..argument_size {
                                let current_argument = &instruction.arguments[i];
                                match current_argument {
                                    &InstructionArgument::Number(_) => {
                                        if current_argument != &possible_arguments[i] {
                                            same_arguments = false;
                                            break;
                                        }
                                    }
                                    &InstructionArgument::Numbers(sizes) => {
                                        let mut found_size = false;
                                        for size in sizes {
                                            if let InstructionArgument::Number(number_size) =
                                                possible_arguments[i]
                                            {
                                                if size == &number_size {
                                                    found_size = true;
                                                    break;
                                                }
                                            }
                                        }

                                        if !found_size {
                                            same_arguments = false;
                                            break;
                                        }
                                    }
                                    &InstructionArgument::Register(register_name) => {
                                        if let InstructionArgument::NotStaticRegister(
                                            ref possible_register,
                                        ) = possible_arguments[i]
                                        {
                                            if register_name != possible_register {
                                                same_arguments = false;
                                                break;
                                            }
                                        } else {
                                            same_arguments = false;
                                            break;
                                        }
                                    }
                                    _ => continue,
                                };
                            }
                        }

                        if same_arguments {
                            return Some(instruction);
                        }
                    }
                }
            }
        }

        return None;
    }

    fn add_error_message(&mut self, error_message: &str, offending_token: Token) {
        let new_message = ErrorMessage {
            message: error_message.to_owned(),
            token: offending_token,
            severity: ErrorSeverity::Error,
        };

        self.error_messages.push(new_message);
    }

    fn add_to_argument_list_capture_register(
        &mut self,
        argument_list: &mut Vec<InstructionArgument>,
        argument: &ParseArgument,
    ) -> Option<String> {
        match argument {
            &ParseArgument::NumberLiteral(number) => {
                argument_list.push(InstructionArgument::Number(number.argument_size));
                return None;
            }
            &ParseArgument::Register(ref register_name) => {
                let result_register_name = register_name.to_owned();
                argument_list.push(InstructionArgument::NotStaticRegister(
                    register_name.to_owned(),
                ));
                return Some(result_register_name);
            }
            &ParseArgument::Identifier(_) => {
                return None;
            }
        };
    }

    fn add_to_argument_list(
        &mut self,
        argument_list: &mut Vec<InstructionArgument>,
        argument: &ParseArgument,
    ) {
        match argument {
            &ParseArgument::NumberLiteral(number) => {
                argument_list.push(InstructionArgument::Number(number.argument_size));
            }
            &ParseArgument::Register(ref register_name) => {
                argument_list.push(InstructionArgument::NotStaticRegister(
                    register_name.to_owned(),
                ));
            }
            &ParseArgument::Identifier(_) => {}
        };
    }
}

impl TreePass for InstructionToStatementPass {
    fn has_errors(&self) -> bool {
        return !self.error_messages.is_empty();
    }

    fn get_error_messages(&self) -> &Vec<ErrorMessage> {
        &self.error_messages
    }

    fn do_pass(
        &mut self,
        parse_tree: Vec<ParseNode>,
        _symbol_table: &mut SymbolTable,
    ) -> Vec<ParseNode> {
        let mut new_tree: Vec<ParseNode> = Vec::new();

        for node in parse_tree.iter() {
            match node.expression {
                ParseExpression::ImpliedInstruction(ref opcode_name) => {
                    match self.find_suitable_instruction(
                        opcode_name,
                        &[AddressingMode::Implied],
                        &[],
                    ) {
                        Some(instruction) => {
                            new_tree.push(ParseNode {
                                start_token: node.start_token.clone(),
                                expression: ParseExpression::FinalInstruction(
                                    FinalInstruction::ImpliedInstruction(instruction),
                                ),
                            });
                        }
                        None => {
                            self.add_error_message(
                                &format!(
                                    "opcode '{}' does not support implied addressing mode.",
                                    opcode_name
                                ),
                                node.start_token.clone(),
                            );
                            new_tree.push(node.clone());
                        }
                    }
                }
                ParseExpression::ImmediateInstruction(ref opcode_name, ref argument) => {
                    match argument {
                        &ParseArgument::NumberLiteral(number) => {
                            match self.find_suitable_instruction(
                                opcode_name,
                                &[AddressingMode::Immediate],
                                &[InstructionArgument::Number(number.argument_size)],
                            ) {
                                Some(instruction) => {
                                    new_tree.push(ParseNode {
                                        start_token: node.start_token.clone(),
                                        expression: ParseExpression::FinalInstruction(
                                            FinalInstruction::SingleArgumentInstruction(
                                                instruction,
                                                argument.clone(),
                                            ),
                                        ),
                                    });
                                }
                                None => {
                                    self.add_error_message(&format!("opcode '{}' does not support immediate addressing mode of size {}-bit.", opcode_name, argument_size_to_bit_size(number.argument_size)), node.start_token.clone());
                                    new_tree.push(node.clone());
                                }
                            }
                        }
                        &ParseArgument::Register(ref register_name) => {
                            self.add_error_message(&format!("immediate addressing mode does not support '{}' register argument.", register_name), node.start_token.clone());
                            new_tree.push(node.clone());
                        }
                        &ParseArgument::Identifier(_) => {
                            new_tree.push(node.clone());
                        }
                    }
                }
                ParseExpression::SingleArgumentInstruction(ref opcode_name, ref argument) => {
                    match argument {
                        &ParseArgument::NumberLiteral(number) => {
                            match self.find_suitable_instruction(
                                opcode_name,
                                &[AddressingMode::SingleArgument, AddressingMode::Relative],
                                &[InstructionArgument::Number(number.argument_size)],
                            ) {
                                Some(instruction) => {
                                    new_tree.push(ParseNode {
                                        start_token: node.start_token.clone(),
                                        expression: ParseExpression::FinalInstruction(
                                            FinalInstruction::SingleArgumentInstruction(
                                                instruction,
                                                argument.clone(),
                                            ),
                                        ),
                                    });
                                }
                                None => {
                                    self.add_error_message(
                                        &format!(
                                            "opcode '{}' does not support {} addressing mode.",
                                            opcode_name,
                                            (&self.system.size_to_addressing_mode)(
                                                number.argument_size
                                            )
                                        ),
                                        node.start_token.clone(),
                                    );
                                    new_tree.push(node.clone());
                                }
                            }
                        }
                        &ParseArgument::Register(ref register_name) => {
                            self.add_error_message(
                                &format!(
                                    "addressing mode does not support '{}' register argument.",
                                    register_name
                                ),
                                node.start_token.clone(),
                            );
                            new_tree.push(node.clone());
                        }
                        &ParseArgument::Identifier(_) => {
                            new_tree.push(node.clone());
                        }
                    }
                }
                ParseExpression::IndexedInstruction(
                    ref opcode_name,
                    ref argument1,
                    ref argument2,
                ) => {
                    let mut argument_list = Vec::new();
                    let mut result_register_name = String::new();

                    match self.add_to_argument_list_capture_register(&mut argument_list, &argument1)
                    {
                        Some(result) => result_register_name = result,
                        _ => {}
                    }

                    match self.add_to_argument_list_capture_register(&mut argument_list, &argument2)
                    {
                        Some(result) => result_register_name = result,
                        _ => {}
                    }

                    match self.find_suitable_instruction(
                        opcode_name,
                        &[AddressingMode::Indexed],
                        &argument_list,
                    ) {
                        Some(instruction) => {
                            new_tree.push(ParseNode {
                                start_token: node.start_token.clone(),
                                expression: ParseExpression::FinalInstruction(
                                    FinalInstruction::SingleArgumentInstruction(
                                        instruction,
                                        argument1.clone(),
                                    ),
                                ),
                            });
                        }
                        None => {
                            if result_register_name == "s" {
                                self.add_error_message(
                                    &format!(
                                        "opcode '{}' does not support stack relative mode.",
                                        opcode_name
                                    ),
                                    node.start_token.clone(),
                                );
                            } else {
                                self.add_error_message(&format!("opcode '{}' does not support '{}' indexed addressing mode.", opcode_name, result_register_name), node.start_token.clone());
                            }
                            new_tree.push(node.clone());
                        }
                    }
                }
                ParseExpression::IndirectInstruction(ref opcode_name, ref argument) => {
                    match argument {
                        &ParseArgument::NumberLiteral(number) => {
                            match self.find_suitable_instruction(
                                opcode_name,
                                &[AddressingMode::Indirect],
                                &[InstructionArgument::Number(number.argument_size)],
                            ) {
                                Some(instruction) => {
                                    new_tree.push(ParseNode {
                                        start_token: node.start_token.clone(),
                                        expression: ParseExpression::FinalInstruction(
                                            FinalInstruction::SingleArgumentInstruction(
                                                instruction,
                                                argument.clone(),
                                            ),
                                        ),
                                    });
                                }
                                None => {
                                    self.add_error_message(&format!("opcode '{}' does not support indirect addressing mode.", opcode_name), node.start_token.clone());
                                    new_tree.push(node.clone());
                                }
                            }
                        }
                        &ParseArgument::Register(ref register_name) => {
                            self.add_error_message(
                                &format!(
                                    "addressing mode does not support '{}' register argument.",
                                    register_name
                                ),
                                node.start_token.clone(),
                            );
                            new_tree.push(node.clone());
                        }
                        &ParseArgument::Identifier(_) => {
                            new_tree.push(node.clone());
                        }
                    }
                }
                ParseExpression::IndirectLongInstruction(ref opcode_name, ref argument) => {
                    match argument {
                        &ParseArgument::NumberLiteral(number) => {
                            match self.find_suitable_instruction(
                                opcode_name,
                                &[AddressingMode::IndirectLong],
                                &[InstructionArgument::Number(number.argument_size)],
                            ) {
                                Some(instruction) => {
                                    new_tree.push(ParseNode {
                                        start_token: node.start_token.clone(),
                                        expression: ParseExpression::FinalInstruction(
                                            FinalInstruction::SingleArgumentInstruction(
                                                instruction,
                                                argument.clone(),
                                            ),
                                        ),
                                    });
                                }
                                None => {
                                    self.add_error_message(&format!("opcode '{}' does not support indirect long addressing mode.", opcode_name), node.start_token.clone());
                                    new_tree.push(node.clone());
                                }
                            }
                        }
                        &ParseArgument::Register(ref register_name) => {
                            self.add_error_message(
                                &format!(
                                    "addressing mode does not support '{}' register argument.",
                                    register_name
                                ),
                                node.start_token.clone(),
                            );
                            new_tree.push(node.clone());
                        }
                        &ParseArgument::Identifier(_) => {
                            new_tree.push(node.clone());
                        }
                    }
                }
                ParseExpression::IndexedIndirectInstruction(
                    ref opcode_name,
                    ref argument1,
                    ref argument2,
                ) => {
                    let mut argument_list = Vec::new();
                    let mut result_register_name = String::new();

                    match self.add_to_argument_list_capture_register(&mut argument_list, &argument1)
                    {
                        Some(result) => result_register_name = result,
                        None => {}
                    }

                    match self.add_to_argument_list_capture_register(&mut argument_list, &argument2)
                    {
                        Some(result) => result_register_name = result,
                        None => {}
                    }

                    match self.find_suitable_instruction(
                        opcode_name,
                        &[AddressingMode::IndexedIndirect],
                        &argument_list,
                    ) {
                        Some(instruction) => {
                            new_tree.push(ParseNode {
                                start_token: node.start_token.clone(),
                                expression: ParseExpression::FinalInstruction(
                                    FinalInstruction::SingleArgumentInstruction(
                                        instruction,
                                        argument1.clone(),
                                    ),
                                ),
                            });
                        }
                        None => {
                            self.add_error_message(&format!("opcode '{}' does not support '{}' indexed indirect addressing mode.", opcode_name, result_register_name), node.start_token.clone());
                            new_tree.push(node.clone());
                        }
                    }
                }
                ParseExpression::IndirectIndexedInstruction(
                    ref opcode_name,
                    ref argument1,
                    ref argument2,
                ) => {
                    let mut argument_list = Vec::new();
                    let mut result_register_name = String::new();

                    match self.add_to_argument_list_capture_register(&mut argument_list, &argument1)
                    {
                        Some(result) => result_register_name = result,
                        None => {}
                    }

                    match self.add_to_argument_list_capture_register(&mut argument_list, &argument2)
                    {
                        Some(result) => result_register_name = result,
                        None => {}
                    }

                    match self.find_suitable_instruction(
                        opcode_name,
                        &[AddressingMode::IndirectIndexed],
                        &argument_list,
                    ) {
                        Some(instruction) => {
                            new_tree.push(ParseNode {
                                start_token: node.start_token.clone(),
                                expression: ParseExpression::FinalInstruction(
                                    FinalInstruction::SingleArgumentInstruction(
                                        instruction,
                                        argument1.clone(),
                                    ),
                                ),
                            });
                        }
                        None => {
                            self.add_error_message(&format!("opcode '{}' does not support '{}' indirect indexed addressing mode.", opcode_name, result_register_name), node.start_token.clone());
                            new_tree.push(node.clone());
                        }
                    }
                }
                ParseExpression::IndirectIndexedLongInstruction(
                    ref opcode_name,
                    ref argument1,
                    ref argument2,
                ) => {
                    let mut argument_list = Vec::new();
                    let mut result_register_name = String::new();

                    match self.add_to_argument_list_capture_register(&mut argument_list, &argument1)
                    {
                        Some(result) => result_register_name = result,
                        None => {}
                    }

                    match self.add_to_argument_list_capture_register(&mut argument_list, &argument2)
                    {
                        Some(result) => result_register_name = result,
                        None => {}
                    }

                    match self.find_suitable_instruction(
                        opcode_name,
                        &[AddressingMode::IndirectIndexedLong],
                        &argument_list,
                    ) {
                        Some(instruction) => {
                            new_tree.push(ParseNode {
                                start_token: node.start_token.clone(),
                                expression: ParseExpression::FinalInstruction(
                                    FinalInstruction::SingleArgumentInstruction(
                                        instruction,
                                        argument1.clone(),
                                    ),
                                ),
                            });
                        }
                        None => {
                            self.add_error_message(&format!("opcode '{}' does not support '{}' indirect indexed long addressing mode.", opcode_name, result_register_name), node.start_token.clone());
                            new_tree.push(node.clone());
                        }
                    }
                }
                ParseExpression::BlockMoveInstruction(
                    ref opcode_name,
                    ref argument1,
                    ref argument2,
                ) => {
                    let mut argument_list = Vec::new();

                    self.add_to_argument_list(&mut argument_list, &argument1);
                    self.add_to_argument_list(&mut argument_list, &argument2);

                    match self.find_suitable_instruction(
                        opcode_name,
                        &[AddressingMode::BlockMove],
                        &argument_list,
                    ) {
                        Some(instruction) => {
                            new_tree.push(ParseNode {
                                start_token: node.start_token.clone(),
                                expression: ParseExpression::FinalInstruction(
                                    FinalInstruction::TwoArgumentInstruction(
                                        instruction,
                                        argument1.clone(),
                                        argument2.clone(),
                                    ),
                                ),
                            });
                        }
                        None => {
                            self.add_error_message(
                                &format!(
                                    "opcode '{}' does not support block mode addressing mode.",
                                    opcode_name
                                ),
                                node.start_token.clone(),
                            );
                            new_tree.push(node.clone());
                        }
                    }
                }
                ParseExpression::StackRelativeIndirectIndexedInstruction(
                    ref opcode_name,
                    ref argument1,
                    ref argument2,
                    ref argument3,
                ) => {
                    let mut argument_list = Vec::new();

                    self.add_to_argument_list(&mut argument_list, &argument1);
                    self.add_to_argument_list(&mut argument_list, &argument2);
                    self.add_to_argument_list(&mut argument_list, &argument3);

                    match self.find_suitable_instruction(
                        opcode_name,
                        &[AddressingMode::StackRelativeIndirectIndexed],
                        &argument_list,
                    ) {
                        Some(instruction) => {
                            new_tree.push(ParseNode {
                                start_token: node.start_token.clone(),
                                expression: ParseExpression::FinalInstruction(
                                    FinalInstruction::SingleArgumentInstruction(
                                        instruction,
                                        argument1.clone(),
                                    ),
                                ),
                            });
                        }
                        None => {
                            self.add_error_message(&format!("opcode '{}' does not support stack relative indirect indexed addressing mode.", opcode_name), node.start_token.clone());
                            new_tree.push(node.clone());
                        }
                    }
                }
                _ => {
                    new_tree.push(node.clone());
                }
            };
        }

        return new_tree;
    }
}
