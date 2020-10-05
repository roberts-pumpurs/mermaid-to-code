
#[derive(Debug, Clone, PartialEq)]
pub struct ASTAttribute {
    data_type: String,
    name: String,
}

impl ASTAttribute {
    pub fn new(data_type: String, name: String) -> Self {
        Self { data_type, name }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ASTFunction {
    return_type: String,
    name: String,
    parameters: Vec<ASTAttribute>,
}

impl ASTFunction {
    pub fn new(return_type: String, name: String, parameters: Vec<ASTAttribute>) -> Self {
        Self {
            return_type,
            name,
            parameters,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ASTClass {
    pub attributes: Vec<ASTAttribute>,
    pub functions: Vec<ASTFunction>,
    pub inheritance: Vec<ASTClass>,
    pub name: String,
}

impl ASTClass {
    pub fn new(
        name: String,
    ) -> Self {
        Self {
            attributes: Vec::new(),
            functions: Vec::new(),
            inheritance: Vec::new(),
            name,
        }
    }
}
