use crate::ast::{ASTAttribute, ASTClass, ASTFunction};

pub trait Parsable {
    fn imports(&self) -> &str;
    fn parse_class(&self, class: &ASTClass) -> String;
    fn parse_attribute(&self, attribute: &ASTAttribute) -> String;
    fn parse_function(&self, function: &ASTFunction) -> String;
    fn parse_type(&self, data_type: &DataType) -> String;
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    STRING,
    BOOL,
    FLOAT,
    DATETIME,
    DOUBLE,
    CHAR,
    INTEGER,
    // Foreign key to another class
    FOREIGNKEY(String),
}

