extern crate clap;

mod zeal;
mod snes_cpu;

use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::prelude::*;

use snes_cpu::*;
use zeal::lexer::*;
use zeal::system_definition::SystemDefinition;

static SUPPORTED_SYSTEMS: &'static [&'static SystemDefinition] = &[
    &SNES_CPU
];

fn absolute_path(path: &Path) -> std::io::Result<PathBuf> {
    let path_buf = path.canonicalize()?;

    #[cfg(windows)]
    let path_buf = Path::new(path_buf.as_path().to_string_lossy().trim_left_matches(r"\\?\")).to_path_buf();

    Ok(path_buf)
}

fn main() {
    let zeal_args_info = App::new("Zeal Compiler")
        .version("0.1.0")
        .author("Michaël Larouche <michael.larouche@gmail.com>")
        .about("Compiler/Assembler for SNES/SFC 65816")
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .takes_value(true)
                .help("Resultant ROM file or an existing rom file"),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("Input assembler file")
                .index(1),
        )
        .arg(
            Arg::with_name("cpu")
                .short("c")
                .long("cpu")
                .help("CPU type to use.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("listcpu")
                .long("list-cpu")
                .help("List available CPU types.")
        );

    let cmd_matches = zeal_args_info.get_matches();

    if cmd_matches.is_present("listcpu") {
        println!("Available CPU:");

        for system in SUPPORTED_SYSTEMS.iter() {
            println!("* {}: {}", system.short_name, system.name);
        }
        std::process::exit(0);
    }

    let input_file = match cmd_matches.value_of("INPUT") {
        None => {
            println!("ERROR: No input file found!\n");
            println!("{}", cmd_matches.usage());
            std::process::exit(0);
        },
        Some(result) => result
    };

    let input_path = Path::new(input_file);
    let path_display = input_path.display();

    let mut file = match File::open(Path::new(input_file)) {
        Err(why) => panic!("Couldn't open {}: {}", path_display, why.description()),
        Ok(file) => file,
    };

    let mut file_contents = String::new();
    match file.read_to_string(&mut file_contents) {
        Err(why) => panic!("Couldn't read {}: {}", path_display, why.description()),
        Ok(result) => result
    };

    let file_string_path = match absolute_path(input_path) {
        Err(_) => std::path::PathBuf::new(),
        Ok(result) => result
    };

    let mut lexer = Lexer::new(&file_contents, file_string_path.to_str().unwrap().to_string());

    loop {
        let token = lexer.get_next_token();
        match token.ttype {
            TokenType::Invalid(invalid_char) => println!("Invalid token '{}' found in {} at line {} column {}", invalid_char, token.source_file, token.line, token.start_column),
            TokenType::Identifier(identifier) => println!("Identifier: {}", identifier),
            TokenType::EndOfFile => {
                println!("Done!");
                break;
            }
        }
    }
}
