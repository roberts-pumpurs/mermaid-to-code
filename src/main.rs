mod ast;
mod common;
mod languages;
mod output;

#[macro_use]
extern crate clap;
use crate::common::Parsable;
use crate::output::generate_data;
use ast::parse_to_ast;
use clap::{App, ArgMatches};
use languages::Django;

use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // ------------------Arg extracting-------------------//
    let file_path_to_parse = matches.value_of("input-file").unwrap();
    let language_code_to_generate = matches.value_of("output-language").unwrap();
    let output_file_path = matches.value_of("output-file").unwrap();

    // let output_construction = get_language_server(language_code_to_generate);

    // ------------------File reading-------------------- //
    let mut file = File::open(file_path_to_parse)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // ----------Parsing the data-----//
    let ast_classes = parse_to_ast(&contents);
    let final_output = generate_data(&ast_classes, Django::new());
    // ------------------File output----------------------//

    let mut file = File::create(&output_file_path)?;
    file.write_all(final_output.as_bytes())?;

    Ok(())
}
