use std::{collections::HashMap, fmt::Display};

#[derive(Debug)]
pub struct StructInfo {
    pub name: String,
    pub fields: HashMap<String, String>,
}

impl Display for StructInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fields = Vec::new();

        for (name, param_type) in &self.fields {
            fields.push(format!("    {}: {},", name, param_type));
        }

        let fields = fields.join("\n");

        write!(f, "pub struct {} {{\n{}\n}}", self.name, fields,)
    }
}
