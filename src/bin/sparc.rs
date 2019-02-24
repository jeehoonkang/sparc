extern crate clap;
extern crate sparc;

use clap::{App, Arg};
use sparc::Executor;
use std::fs;

fn main() {
    let matches = App::new("SPARC Interpreter")
        .version("0.1")
        .author("Jeehoon Kang <jeehoon.kang@sf.snu.ac.kr>")
        .about("Execute SPARC program")
        .arg(
            Arg::with_name("INPUT_FILE")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)
    let input_file = matches.value_of("INPUT_FILE").unwrap();
    let contents =
        fs::read_to_string(input_file).expect(&format!("Cannot read from the file {}", input_file));

    let executor = Executor::new();
    match executor.exec(&contents) {
        Ok(value) => {
            println!("{:?}", value);
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}
