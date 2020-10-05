mod structs;

use regex::Regex;
use structs::{ASTClass, ASTFunction, ASTAttribute};

pub fn parse_to_ast(input: &str) -> Vec<ASTClass> {
    let lines = input.split("\n");

    // ------------------Define static regexes-------------------//
    let class_finder = Regex::new(r"(\w+\{|\w+\s\{)").unwrap();
    let function_or_attribute = Regex::new(r"\w+(\(.*\)|)").unwrap();

    // ------------------State machine variables-------------------//
    let mut result = Vec::new();
    let mut empty_class = ASTClass::new("".to_owned());
    let mut parsing_class = false;
    // ------------------Iterate over the lines-------------------//
    for line in lines {
        println!("parsing_class {:?}", parsing_class);
        if line.contains("}") {
            parsing_class = false;
            result.push(empty_class.clone());
            empty_class = ASTClass::new("".to_owned());
        }
        if parsing_class {
            let captures = function_or_attribute.captures(&line);
            // let mut data_type = String::new();
            // let mut name = String::new();
            // let mut params = Vec::new();
            match captures {
                Some(capture) => {
                    if line.contains("(") {
                        // TODO handle functions
                    } else {
                        let data_type  = capture[0].to_owned();
                        let name  = capture[1].to_owned();
                        let attribute_line = ASTAttribute::new(data_type, name);
                        empty_class.attributes.push(attribute_line)
                    }

                }
                None => {}
            }
        } else if line.contains("class") {
            for matching_name in class_finder.captures_iter(&line) {
                empty_class.name = matching_name[0].to_owned().trim_matches('{').to_owned();
            }
            parsing_class = true;
        }
    }

    result
}
