extern crate clap;

mod zeal;
mod snes_cpu;

use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::prelude::*;

use snes_cpu::*;

use zeal::collect_label_pass::*;
use zeal::instruction_statement_pass::*;
use zeal::lexer::*;
use zeal::output_writer::*;
use zeal::parser::*;
use zeal::pass::*;
use zeal::resolve_label_pass::*;
use zeal::symbol_table::*;
use zeal::system_definition::SystemDefinition;

static SUPPORTED_SYSTEMS: &'static [&'static SystemDefinition] = &[&SNES_CPU];

fn absolute_path(path: &Path) -> std::io::Result<PathBuf> {
    let path_buf = path.canonicalize()?;

    #[cfg(windows)]
    let path_buf = Path::new(
        path_buf
            .as_path()
            .to_string_lossy()
            .trim_left_matches(r"\\?\"),
    ).to_path_buf();

    Ok(path_buf)
}

fn find_system(cpu_name: &str) -> &'static SystemDefinition {
    for system in SUPPORTED_SYSTEMS.iter() {
        if system.short_name == cpu_name {
            return system;
        }
    }

    &SNES_CPU
}

fn print_error_message(error_message: &ErrorMessage) {
    let severity_string = match error_message.severity {
        ErrorSeverity::Error => "error",
        ErrorSeverity::Warning => "warning",
    };

    println!(
        "{}({},{}): {}: {}",
        error_message.token.source_file,
        error_message.token.line,
        error_message.token.start_column,
        severity_string,
        error_message.message
    );

    for context_char in error_message.token.context_start.clone() {
        if context_char == '\n' {
            break;
        } else {
            print!("{}", context_char);
        }
    }
    println!("");

    for _ in 0..(error_message.token.start_column - 1) {
        print!(" ");
    }

    for _ in error_message.token.start_column..error_message.token.end_column {
        print!("^");
    }

    println!("");
}

fn process_errors(messages: &Vec<ErrorMessage>) {
    for error_message in messages {
        print_error_message(&error_message);
    }

    for error_message in messages {
        if error_message.severity == ErrorSeverity::Error {
            std::process::exit(1);
        }
    }
}

fn main() {
    let zeal_args_info = App::new("Zeal Compiler")
        .version("0.1.0")
        .author("Michaël Larouche <michael.larouche@gmail.com>")
        .about("Compiler/Assembler for SNES/SFC 65816 (for now)")
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .takes_value(true)
                .required(true)
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
                .help("CPU type to use. (Default: snes-cpu)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("listcpu")
                .long("list-cpu")
                .help("List available CPU types."),
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
            std::process::exit(1);
        }
        Some(result) => result,
    };

    let output_path = match cmd_matches.value_of("output") {
        None => {
            println!("ERROR: No output file found!\n");
            println!("{}", cmd_matches.usage());
            std::process::exit(1);
        }
        Some(result) => Path::new(result),
    };

    let input_path = Path::new(input_file);
    let path_display = input_path.display();

    let mut file = match File::open(input_path) {
        Err(why) => panic!("Couldn't open {}: {}", path_display, why.description()),
        Ok(file) => file,
    };

    let mut file_contents = String::new();
    match file.read_to_string(&mut file_contents) {
        Err(why) => panic!("Couldn't read {}: {}", path_display, why.description()),
        Ok(result) => result,
    };

    let file_string_path = match absolute_path(input_path) {
        Err(_) => std::path::PathBuf::new(),
        Ok(result) => result,
    };

    let selected_cpu = match cmd_matches.value_of("cpu") {
        None => &SNES_CPU,
        Some(cpu_name) => find_system(cpu_name),
    };

    let lexer = Lexer::new(
        selected_cpu,
        &file_contents,
        file_string_path.to_str().unwrap().to_string(),
    );

    let mut parser = Parser::new(lexer);
    let parse_tree = parser.parse_tree();
    if parser.has_errors() {
        process_errors(&parser.error_messages);
    }

    let mut symbol_table = SymbolTable::new();

    let mut collect_label_pass = CollectLabelPass::new(selected_cpu,);
    let collect_label_tree = collect_label_pass.do_pass(&parse_tree,  &mut symbol_table);
    if collect_label_pass.has_errors() {
        process_errors(&collect_label_pass.get_error_messages());
    }

    let mut resolve_label_pass = ResolveLabelPass::new(selected_cpu);
    let resolve_label_tree = resolve_label_pass.do_pass(&collect_label_tree,  &mut symbol_table);
    if resolve_label_pass.has_errors() {
        process_errors(&resolve_label_pass.get_error_messages());
    }

    let mut instruction_statement_pass = InstructionToStatementPass::new(selected_cpu);
    let instruction_tree = instruction_statement_pass.do_pass(&resolve_label_tree,  &mut symbol_table);
    if instruction_statement_pass.has_errors() {
        process_errors(&instruction_statement_pass.get_error_messages());
    }

    let mut output_writer = OutputWriter::new(selected_cpu, output_path);
    output_writer.write(&instruction_tree);
}
