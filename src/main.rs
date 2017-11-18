extern crate clap;

use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

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
        );

    let cmd_matches = zeal_args_info.get_matches();

    if !cmd_matches.is_present("INPUT") {
        println!("ERROR: No input file found!\n");
        println!("{}", cmd_matches.usage());
        std::process::exit(0);
    }

    let input_file = cmd_matches.value_of("INPUT").unwrap();
    println!("The input file is: {}", input_file);

    let input_path = Path::new(input_file);
    let path_display = input_path.display();

    let mut file = match File::open(Path::new(input_file)) {
        Err(why) => panic!("Couldn't open {}: {}", path_display, why.description()),
        Ok(file) => file,
    };

    let mut file_contents = String::new();
    match file.read_to_string(&mut file_contents) {
        Err(why) => panic!("Couldn't read {}: {}", path_display, why.description()),
        Ok(_) => print!("{}", file_contents),
    }
}
