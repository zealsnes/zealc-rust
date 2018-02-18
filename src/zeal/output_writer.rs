extern crate byteorder;

use self::byteorder::{BigEndian, LittleEndian, WriteBytesExt};
use std::error::Error;
use std::io::{BufReader, Read, Seek, SeekFrom, Write};
use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;
use zeal::lexer::*;
use zeal::parser::*;
use zeal::system_definition::*;

pub struct OutputWriter {
    system: &'static SystemDefinition,
    output: File,
    map_function: fn(u32) -> u32,
}

fn map_default(value: u32) -> u32 {
    value
}

fn map_snes_lorom(value: u32) -> u32 {
    ((value & 0x7F0000) >> 1) | (value & 0x7FFF)
}

fn map_snes_hirom(value: u32) -> u32 {
    value & 0x3FFFFF
}

impl<'a> OutputWriter {
    pub fn new(system: &'static SystemDefinition, file_path: &Path) -> Self {
        let mut file_options = OpenOptions::new();
        file_options.write(true);
        file_options.create_new(true);

        let file = match file_options.open(file_path) {
            Ok(file) => file,
            Err(_) => File::create(file_path).unwrap(),
        };

        OutputWriter {
            system: system,
            output: file,
            map_function: map_default
        }
    }

    pub fn write(&mut self, parse_tree: &Vec<ParseNode>) {
        for node in parse_tree.iter() {
            match node.expression {
                ParseExpression::FinalInstruction(ref final_instruction) => {
                    self.handle_final_instruction(final_instruction);
                }
                ParseExpression::IncBinStatement(ref filename, _) => {
                    self.do_incbin(&filename);
                }
                ParseExpression::OriginStatement(ref number) => {
                    let physical_address = (self.map_function)(number.number);
                    match self.output.seek(SeekFrom::Start(physical_address as u64)) {
                        _=> {}
                    }
                }
                ParseExpression::SnesMapStatement(ref map_mode) => {
                    match map_mode {
                        &SnesMap::LoRom => self.map_function = map_snes_lorom,
                        &SnesMap::HiRom => self.map_function = map_snes_hirom,
                    };
                }
                _ => {}
            };
        }
    }

    fn handle_final_instruction(&mut self, final_instruction: &FinalInstruction) {
        match final_instruction {
            &FinalInstruction::ImpliedInstruction(instruction) => {
                self.output.write_u8(instruction.opcode).unwrap();
            }
            &FinalInstruction::SingleArgumentInstruction(instruction, ref argument) => {
                self.output.write_u8(instruction.opcode).unwrap();

                match argument {
                    &ParseArgument::NumberLiteral(ref number) => self.write_number_literal(&number),
                    _ => {}
                }
            }
            &FinalInstruction::TwoArgumentInstruction(instruction, ref argument1, ref argument2) => {
                self.output.write_u8(instruction.opcode).unwrap();

                match argument1 {
                    &ParseArgument::NumberLiteral(ref number) => self.write_number_literal(&number),
                    _ => {}
                };

                match argument2 {
                    &ParseArgument::NumberLiteral(ref number) => self.write_number_literal(&number),
                    _ => {}
                };
            }
        }
    }

    fn write_number_literal(&mut self, number: &NumberLiteral) {
        let is_big_endian = self.system.is_big_endian;

        if is_big_endian {
            match number.argument_size {
                ArgumentSize::Word8 => self.output.write_u8(number.number as u8).unwrap(),
                ArgumentSize::Word16 => self.output
                    .write_u16::<BigEndian>(number.number as u16)
                    .unwrap(),
                ArgumentSize::Word24 => self.output.write_u24::<BigEndian>(number.number).unwrap(),
                ArgumentSize::Word32 => self.output.write_u32::<BigEndian>(number.number).unwrap(),
            };
        } else {
            match number.argument_size {
                ArgumentSize::Word8 => self.output.write_u8(number.number as u8).unwrap(),
                ArgumentSize::Word16 => self.output
                    .write_u16::<LittleEndian>(number.number as u16)
                    .unwrap(),
                ArgumentSize::Word24 => self.output
                    .write_u24::<LittleEndian>(number.number)
                    .unwrap(),
                ArgumentSize::Word32 => self.output
                    .write_u32::<LittleEndian>(number.number)
                    .unwrap(),
            };
        }
    }

    fn do_incbin(&mut self, filename: &str) {
        let input_path = Path::new(filename);
        let path_display = input_path.display();

        let file = match File::open(input_path) {
            Err(why) => panic!("Couldn't open {}: {}", path_display, why.description()),
            Ok(file) => file,
        };

        let mut buf_reader = BufReader::new(file);
        let mut file_content: Vec<u8> = Vec::new();

        buf_reader.read_to_end(&mut file_content).unwrap();

        self.output.write(&file_content).unwrap();
    }
}
