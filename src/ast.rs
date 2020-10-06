mod structs;

use std::collections::HashMap;

use regex::Regex;
use structs::{ASTAttribute, ASTClass, ASTFunction};

pub fn parse_to_ast(input: &str) -> HashMap<String, ASTClass> {
    let lines = input.split("\n");

    // ------------------Define static regexes-------------------//
    let class_finder = Regex::new(r"(\w+\{|\w+\s\{)").unwrap();
    let function_or_attribute = Regex::new(r"\w+(\(.*\)|)").unwrap();

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
                empty_class.name = matching_name[0].to_owned().trim_matches('{').to_owned();
            }
            parsing_class = true;
        }
        if parsing_class {
            let captures = function_or_attribute.captures(&line);
            match captures {
                Some(capture) => {
                    if line.contains("(") {
                        // TODO handle functions
                    } else {
                        println!("{:?}", capture);
                        let data_type = capture[0].to_owned();
                        let name = capture[1].to_owned();
                        let attribute_line = ASTAttribute::new(data_type, name);
                        empty_class.attributes.push(attribute_line)
                    }
                }
                None => {}
            }
        }
    }

    result
}
