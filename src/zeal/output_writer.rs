use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use zeal::parser::*;

pub struct OutputWriter<'a> {
    parse_tree: &'a Vec<Expression>,
    output: File
}

impl<'a> OutputWriter<'a> {
    pub fn new(parse_tree: &'a Vec<Expression>, file_path: &Path) -> Self {
        let mut file_options = OpenOptions::new();
        file_options.write(true);

        let file = match file_options.open(file_path) {
            Ok(file) => file,
            Err(_) => File::create(file_path).unwrap()
        };

        OutputWriter {
            parse_tree: parse_tree,
            output: file
        }
    }

    pub fn write(&mut self) {
        for expression in self.parse_tree.iter() {
            match expression {
                &Expression::CpuInstruction(instruction) => {
                    let data = [instruction.opcode];
                    self.output.write(&data);
                }
            };
        }
    }
}