use crate::ast::ASTClass;
use crate::Parsable;
use std::collections::HashMap;

pub fn generate_data<T: Parsable>(input: &HashMap<String, ASTClass>, language: T) -> String {
    let mut result = String::new();
    result.push_str((language.imports().to_owned() + "\n").as_ref());
    let classes_vec_str: Vec<String> = input
        .iter()
        .map(|x| language.parse_class(x.1))
        .collect();
    let classes_str = classes_vec_str.join("\n");
    result.push_str(&classes_str);
    result
}
