use crate::ast::{ASTAttribute, ASTClass, ASTFunction};
use crate::common::DataType;

use crate::common::Parsable;

pub struct Django {
    indent: i32,
}

impl Django {
    pub fn new() -> Self {
        Self { indent: 4 }
    }

    pub fn get_indent(&self) -> String {
        let mut indent = String::new();
        for _ in 0..self.indent {
            indent.push_str(" ");
        }
        indent
    }
}

impl Parsable for Django {
    fn parse_class(&self, class: &ASTClass) -> String {
        // TODO implement subclass parsing
        let indent = self.get_indent();

        let attributes: Vec<String> = class
            .attributes
            .iter()
            .map(|x| self.parse_attribute(x))
            .collect();
        let mut attributes_as_str = attributes.join(&indent);

        let functions: Vec<String> = class
            .functions
            .iter()
            .map(|x| self.parse_function(x))
            .collect();
        let functions_as_str = functions.join(&indent);

        let class_decl = format!("class {name}(models.Model):\n", name = class.name);
        if functions_as_str == "" && attributes_as_str == "" {
            attributes_as_str = format!("{}pass\n", indent);
        }
        format!(
            "{}{}{}{}",
            class_decl, indent, attributes_as_str, functions_as_str
        )
    }

    fn parse_attribute(&self, attribute: &ASTAttribute) -> String {
        let dt_language: String = match &attribute.data_type {
            DataType::STRING => "CharField(max_length=100)".to_owned(),
            DataType::FLOAT => "FloatField()".to_owned(),
            DataType::DATETIME => "DateTimeField()".to_owned(),
            DataType::DOUBLE => "FloatField()".to_owned(),
            DataType::CHAR => "CharField(max_length=1)".to_owned(),
            DataType::FOREIGNKEY(to) => format!("ForeignKey({})", &to),
            DataType::BOOL => "BooleanField()".to_owned(),
            DataType::INTEGER => "IntegerField()".to_owned(),
            DataType::MANYTOMANY(to) => format!("ManyToManyField({})", &to),
        };

        format!(
            "{attributeName} = models.{dataType}\n",
            attributeName = attribute.name,
            dataType = dt_language
        )
    }

    fn parse_function(&self, function: &ASTFunction) -> String {
        let indent = self.get_indent();
        let params: Vec<String> = function
            .parameters
            .iter()
            .map(|x| format!("{}:{}", x.name, self.parse_type(&x.data_type)))
            .collect();
        let params_str = params.join(", ");
        format!(
            "def {name}({params}):\n{indent}pass\n",
            name = function.name,
            params = params_str,
            indent = indent,
        )
    }

    fn parse_type(&self, data_type: &DataType) -> String {
        match data_type {
            DataType::STRING => "str".to_owned(),
            DataType::FLOAT => "flaot".to_owned(),
            DataType::DATETIME => "datetime".to_owned(),
            DataType::DOUBLE => "float".to_owned(),
            DataType::CHAR => "str".to_owned(),
            DataType::BOOL => "bool".to_owned(),
            DataType::INTEGER => "int".to_owned(),
            DataType::FOREIGNKEY(obj) => obj.clone(),
            DataType::MANYTOMANY(obj) => obj.clone(),
        }
    }
    fn imports(&self) -> &str {
        "from django.db import models\n"
    }
}
