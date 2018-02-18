extern crate clap;

mod zeal;
mod snes_cpu;

use clap::{App, Arg};

use std::path::Path;
use std::io::Read;
use std::fs::File;
use std::error::Error;

use snes_cpu::*;

use zeal::collect_label_pass::*;
use zeal::instruction_statement_pass::*;
use zeal::output_writer::*;
use zeal::parser::*;
use zeal::pass::*;
use zeal::resolve_label_pass::*;
use zeal::symbol_table::*;
use zeal::system_definition::SystemDefinition;

static SUPPORTED_SYSTEMS: &'static [&'static SystemDefinition] = &[&SNES_CPU];

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

    let mut file = match File::open(&error_message.token.source_file) {
        Err(why) => panic!(
            "Couldn't open {}: {}",
            error_message.token.source_file,
            why.description()
        ),
        Ok(file) => file,
    };

    let mut string_file_content = String::new();
    match file.read_to_string(&mut string_file_content) {
        Err(why) => panic!(
            "Couldn't read {}: {}",
            error_message.token.source_file,
            why.description()
        ),
        Ok(result) => result,
    };

    for context_char in string_file_content
        .chars()
        .skip(error_message.token.context_start)
    {
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
            Arg::with_name("patch")
                .short("p")
                .long("patch")
                .help("Put the compiler in patching mode. The compiler will only modifiy the relevant parts of the output.")
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

    let selected_cpu = match cmd_matches.value_of("cpu") {
        None => &SNES_CPU,
        Some(cpu_name) => find_system(cpu_name),
    };

    let mut parser = Parser::new(selected_cpu);
    parser.set_current_input_file(input_file);

    let mut parse_tree = parser.parse_tree();
    if parser.has_errors() {
        process_errors(&parser.error_messages);
    }

    let mut symbol_table = SymbolTable::new();

    let mut passes: Vec<Box<TreePass>> = Vec::new();

    passes.push(Box::new(CollectLabelPass::new(selected_cpu)));
    passes.push(Box::new(ResolveLabelPass::new(selected_cpu)));
    passes.push(Box::new(InstructionToStatementPass::new(selected_cpu)));

    for pass in passes.iter_mut() {
        parse_tree = pass.do_pass(parse_tree, &mut symbol_table);
        if pass.has_errors() {
            process_errors(pass.get_error_messages());
        }
    }

    let mut output_options = OutputWriterOptions::new();
    output_options.create_new = !cmd_matches.is_present("patch");

    let mut output_writer = OutputWriter::new(selected_cpu, output_path, &output_options);
    output_writer.write(&parse_tree);
}
