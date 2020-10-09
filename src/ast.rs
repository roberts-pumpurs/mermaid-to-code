pub mod structs;

use std::collections::HashMap;

use regex::Regex;
pub use structs::{ASTAttribute, ASTClass, ASTFunction};

use crate::common::{DataType, RelationEndian};

pub fn parse_to_ast(input: &str) -> Result<HashMap<String, ASTClass>, String> {
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
        if line.contains("}") {
            parsing_class = false;
            result.insert(empty_class.name.clone(), empty_class.clone());
            empty_class = ASTClass::new("".to_owned());
        } else if line.contains("class") {
            for matching_name in class_finder.captures_iter(&line) {
                let name = matching_name[0]
                    .to_owned()
                    .trim_matches('{')
                    .trim()
                    .to_owned().to_uppercase();

                if result.contains_key(&name) {
                    empty_class = result
                        .remove(&name)
                        .ok_or_else(|| "Inconsistent HashMap write/read")?;
                } else {
                    empty_class.name = name;
                }
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

                        let mut dt_enum = DataType::FOREIGNKEY(data_type.clone());
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

        if line.contains("--") {
            let relations: Vec<&str> = line.split(" ").collect();
            let relations_filtered: Vec<String> = relations
                .iter()
                .filter_map(|x| {
                    if x.to_string() != "" {
                        Some(x.trim().to_uppercase().replace("\"", ""))
                    } else {
                        None
                    }
                })
                .collect();
            let left_obj: &str = relations_filtered[0].as_ref();
            let right_obj: &str = relations_filtered[4].as_ref();
            let left_cardinality = convert_cardinality(relations_filtered[1].as_ref()).ok_or_else(|| {
                format!(
                    "Entity relation error, no such relation {}",
                    relations_filtered[1]
                )
            })?;
            let right_cardinality =
                convert_cardinality(relations_filtered[3].as_ref()).ok_or_else(|| {
                    format!(
                        "Entity relation error, no such relation {}",
                        relations_filtered[3]
                    )
                })?;

            if !result.contains_key(left_obj) {
                result.insert(left_obj.to_string(), ASTClass::new(left_obj.to_string()));
            }
            if !result.contains_key(right_obj) {
                result.insert(right_obj.to_string(), ASTClass::new(right_obj.to_string()));
            }

            if left_cardinality == RelationEndian::ONE && right_cardinality == RelationEndian::MANY
            {
                let mut val = result
                    .remove(right_obj)
                    .ok_or_else(|| "Inconsistent HashMap write/read")?;
                val.attributes.push(ASTAttribute::new(
                    DataType::FOREIGNKEY(left_obj.to_string()),
                    format!("{}_fk", left_obj),
                ));
                result.insert(right_obj.to_string(), val);
            } else if left_cardinality == RelationEndian::MANY
                && right_cardinality == RelationEndian::ONE
            {
                let mut val = result
                    .remove(left_obj)
                    .ok_or_else(|| "Inconsistent HashMap write/read")?;
                val.attributes.push(ASTAttribute::new(
                    DataType::FOREIGNKEY(right_obj.to_string()),
                    format!("{}_fk", right_obj),
                ));
                result.insert(left_obj.to_string(), val);
            } else if (left_cardinality == RelationEndian::MANY
                || left_cardinality == RelationEndian::ZEROTILMANY
                || left_cardinality == RelationEndian::ONETILMANY)
                && (right_cardinality == RelationEndian::MANY
                    || right_cardinality == RelationEndian::ZEROTILMANY
                    || right_cardinality == RelationEndian::ONETILMANY)
            {
                let mut val = result
                    .remove(left_obj)
                    .ok_or_else(|| "Inconsistent HashMap write/read")?;
                val.attributes.push(ASTAttribute::new(
                    DataType::MANYTOMANY(right_obj.to_string()),
                    format!("{}_fk", right_obj),
                ));
                result.insert(left_obj.to_string(), val);
            }
        }
    }

    Ok(result)
}

fn convert_cardinality(one_endian: &str) -> Option<RelationEndian> {
    match one_endian.as_ref() {
        "0" => Some(RelationEndian::ZERO),
        "1" => Some(RelationEndian::ONE),
        "N" => Some(RelationEndian::MANY),
        "M" => Some(RelationEndian::MANY),
        "0..N" => Some(RelationEndian::ZEROTILMANY),
        "1..N" => Some(RelationEndian::ONETILMANY),
        "0..M" => Some(RelationEndian::ZEROTILMANY),
        "1..M" => Some(RelationEndian::ONETILMANY),
        "0..1" => Some(RelationEndian::ZEROTILONE),
        _ => None,
    }
}
