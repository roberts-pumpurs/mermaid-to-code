pub mod structs;

use std::collections::HashMap;

use regex::Regex;
pub use structs::{ASTAttribute, ASTClass, ASTFunction};

use crate::common::DataType;

pub fn parse_to_ast(input: &str) -> HashMap<String, ASTClass> {
    let lines_lowercase = input.to_ascii_lowercase();
    let lines = lines_lowercase.split("\n");

    // ------------------Define static regexes-------------------//
    let class_finder = Regex::new(r"(\w+\{|\w+\s\{)").unwrap();
    let function_or_attribute = Regex::new(r"\w+ \w+(\(.*\)|)").unwrap();

    // ------------------State machine variables-------------------//
    let mut result = HashMap::new();
    let mut empty_class = ASTClass::new("".to_owned());
    let mut parsing_class = false;
    // ------------------Iterate over the lines-------------------//
    for line in lines {
        // Skip the `classDiagram` declaration
        if line.contains("classDiagram") {
            continue;
        }
        println!("parsing_class {:?}", parsing_class);
        if line.contains("}") {
            parsing_class = false;
            result.insert(empty_class.name.clone(), empty_class.clone());
            empty_class = ASTClass::new("".to_owned());
        } else if line.contains("class") {
            for matching_name in class_finder.captures_iter(&line) {
                empty_class.name = matching_name[0].to_owned().trim().trim_matches('{').to_owned();
            }
            parsing_class = true;
        } else if parsing_class {
            let captures = function_or_attribute.captures(&line);
            match captures {
                Some(capture) => {
                    if line.contains("(") {
                        // TODO handle functions
                    } else {
                        let split: Vec<&str> = capture[0].split(" ").collect();
                        let data_type = split.get(0).unwrap().to_owned().to_owned();
                        let name = split.get(1).unwrap().to_owned().to_owned();

                        let mut dt_enum  = DataType::FOREIGNKEY(data_type.clone());
                        if data_type == "string" {
                            dt_enum = DataType::STRING;
                        } else if data_type == "float" {
                            dt_enum = DataType::FLOAT;
                        } else if data_type == "int" {
                            dt_enum = DataType::INTEGER;
                        } else if data_type == "bool" {
                            dt_enum = DataType::BOOL;
                        } else if data_type == "datetime" {
                            dt_enum = DataType::DATETIME;
                        } else if data_type == "double" {
                            dt_enum = DataType::DOUBLE;
                        } else if data_type == "char" {
                            dt_enum = DataType::CHAR;
                        }

                        let attribute_line = ASTAttribute::new(dt_enum, name.to_owned());
                        empty_class.attributes.push(attribute_line)
                    }
                }
                None => {}
            }
        }
    }

    result
}
