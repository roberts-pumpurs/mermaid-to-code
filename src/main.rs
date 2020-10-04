
#[macro_use]
extern crate clap;
use clap::{App, ArgMatches};


use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {

    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // ------------------Arg extracting-------------------- //
    let file_path_to_parse = matches.value_of("input-file").unwrap();
    let language_code_to_generate = matches.value_of("output-language").unwrap();
    let output_file_path = matches.value_of("output-file").unwrap();


    // ------------------File reading-------------------- //
    let mut file = File::open(file_path_to_parse)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // ----------Parsing to an abstract language tree----- //
    // ------------------File output-------------------- //
    Ok(())
}
