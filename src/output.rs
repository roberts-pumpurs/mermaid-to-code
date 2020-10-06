use crate::ast::ASTClass;
use std::collections::HashMap;

pub enum OutputTypes {
    Django,
}

pub fn generate_data(input: &HashMap<String, ASTClass>, language: &OutputTypes) -> String {
    let static_types: HashMap<&str, &str> = [
        ("string", "CharField"),
        ("bool", "BooleanField"),
        ("datetime", "DateTimeField"),
        ("float", "FloatField"),
        ("int", "IntegerField"),
    ]
    .iter()
    .cloned()
    .collect();
    let mut result = String::new();
    result.push_str(
        r#"
from django.db import models
"#,
    );
    for (_, parsed_class) in input.iter() {
        let title_class = format!(
            r#"
class {className}(models.Model):"#,
            className = parsed_class.name
        );
        let mut attributes = String::new();
        for attribute in &parsed_class.attributes {
            let dt = attribute.data_type.as_str();
            // let data_type: &str;
            let data_type_try = static_types.get(dt);
            let data_type = match data_type_try {
                Some(val) => val,
                None => dt,
            };
            // let data_type = static_types.get(&attribute.data_type).unwrap().to_string();
            attributes.push_str(&format!(
                r#"
    {attributeName} = models.{dataType}()"#,
                attributeName = attribute.name,
                dataType = data_type
            ))
        }
        result.push_str(&title_class);
        result.push_str(&attributes);
        result.push_str("\n");
    }
    result
}
