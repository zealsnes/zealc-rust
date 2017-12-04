extern crate byteorder;

use self::byteorder::{BigEndian, LittleEndian, WriteBytesExt};
use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;
use zeal::lexer::*;
use zeal::parser::*;
use zeal::system_definition::*;

pub struct OutputWriter {
    system: &'static SystemDefinition,
    output: File,
}

impl<'a> OutputWriter {
    pub fn new(system: &'static SystemDefinition, file_path: &Path) -> Self {
        let mut file_options = OpenOptions::new();
        file_options.write(true);

        let file = match file_options.open(file_path) {
            Ok(file) => file,
            Err(_) => File::create(file_path).unwrap(),
        };

        OutputWriter {
            system: system,
            output: file,
        }
    }

    pub fn write(&mut self, parse_tree: &Vec<ParseNode<'a>>) {
         for node in parse_tree.iter() {
            match node.expression {
                ParseExpression::Statement(ref statement) => {
                    self.handle_statement(statement);
                },
                _ => {}
            };
        }
    }

    fn handle_statement(&mut self, statement: &Statement) {
        match statement {
            &Statement::ImpliedInstruction(instruction) => {
                self.output.write_u8(instruction.opcode).unwrap();
            },
            &Statement::SingleArgumentInstruction(instruction, ref argument) => {
                self.output.write_u8(instruction.opcode).unwrap();

                match argument {
                    &ParseArgument::NumberLiteral(ref number) => {
                        self.write_number_literal(&number)
                    },
                    _ => {}
                }
            },
            &Statement::IndexedInstruction(instruction, ref argument) => {
                self.output.write_u8(instruction.opcode).unwrap();

                match argument {
                    &ParseArgument::NumberLiteral(ref number) => {
                        self.write_number_literal(&number)
                    },
                    _ => {}
                }
            }
        }
    }

    fn write_number_literal(&mut self, number: &NumberLiteral) {
        let is_big_endian = self.system.is_big_endian;

        if is_big_endian {
            match number.argument_size {
                ArgumentSize::Word8 => self.output.write_u8(number.number as u8).unwrap(),
                ArgumentSize::Word16 => self.output.write_u16::<BigEndian>(number.number as u16).unwrap(),
                ArgumentSize::Word24 => self.output.write_u24::<BigEndian>(number.number).unwrap(),
                ArgumentSize::Word32 => self.output.write_u32::<BigEndian>(number.number).unwrap(),
            };
        } else {
            match number.argument_size {
                ArgumentSize::Word8 => self.output.write_u8(number.number as u8).unwrap(),
                ArgumentSize::Word16 => self.output.write_u16::<LittleEndian>(number.number as u16).unwrap(),
                ArgumentSize::Word24 => self.output.write_u24::<LittleEndian>(number.number).unwrap(),
                ArgumentSize::Word32 => self.output.write_u32::<LittleEndian>(number.number).unwrap(),
            };
        }
    }
}
