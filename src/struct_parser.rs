use std::collections::HashMap;

use crate::{
    generic_parser::{extract_name, extract_param, ExtractionError},
    structs::struct_info::StructInfo,
};
use regex::Regex;

pub fn parse_struct(input: &str) -> Result<StructInfo, ExtractionError> {
    let mut fields = HashMap::new();
    let mut lines = input.lines();
    let first_line = match lines.next() {
        None => {
            return Err(ExtractionError {
                line: input.to_string(),
                message: "Failed to read first line!".to_string(),
            });
        }
        Some(first_line) => first_line,
    };
    let struct_name = match extract_name(first_line)? {
        None => {
            return Err(ExtractionError {
                line: first_line.to_string(),
                message: "Failed to parse struct name!".to_string(),
            });
        }
        Some(struct_name) => struct_name,
    };

    for line in lines {
        let param = extract_param(line)?;

        let param = match param {
            None => {
                continue;
            }
            Some(param) => param,
        };

        fields.insert(param.name, param.param_type);
    }

    if struct_name.is_empty() || fields.is_empty() {
        Err(ExtractionError {
            line: input.to_string(),
            message: "Failed to parse struct block!".to_string(),
        })
    } else {
        Ok(StructInfo {
            name: struct_name,
            fields,
        })
    }
}

pub fn varifable_type_to_rust_type(varifable_type: &str) -> Result<String, String> {
    match varifable_type {
        "countable" => Ok("i32".to_string()),
        "wordly" => Ok("String".to_string()),
        _ => Err(format!("Unknown type: {}", varifable_type)),
    }
}
