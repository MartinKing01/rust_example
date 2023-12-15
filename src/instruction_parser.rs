use std::collections::HashMap;

use crate::{
    generic_parser::{
        extract_name, extract_operator, extract_param, extract_variable_name, ExtractionError,
    },
    structs::{
        instruction_info::InstructionInfo,
        operator::{self, Operator},
        struct_info::StructInfo,
    },
};
use regex::Regex;

pub fn parse_instruction(input: &str) -> Result<InstructionInfo, ExtractionError> {
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
    let assignment_name = extract_variable_name(first_line)?;

    let second_line = match lines.next() {
        None => {
            return Err(ExtractionError {
                line: input.to_string(),
                message: "Failed to read second line!".to_string(),
            });
        }
        Some(second_line) => second_line,
    };

    let first_variable = match extract_variable_name(second_line)? {
        None => {
            return Err(ExtractionError {
                line: second_line.to_string(),
                message: "Failed to parse first variable name!".to_string(),
            });
        }
        Some(first_variable) => first_variable,
    };

    let third_line = match lines.next() {
        None => {
            return Err(ExtractionError {
                line: input.to_string(),
                message: "Failed to read third line!".to_string(),
            });
        }
        Some(third_line) => third_line,
    };

    let operator = match extract_operator(third_line)? {
        None => {
            return Err(ExtractionError {
                line: third_line.to_string(),
                message: "Failed to parse operator!".to_string(),
            });
        }
        Some(operator) => operator,
    };

    let fourth_line = match lines.next() {
        None => {
            return Err(ExtractionError {
                line: input.to_string(),
                message: "Failed to read fourth line!".to_string(),
            });
        }
        Some(fourth_line) => fourth_line,
    };

    let second_variable = match extract_variable_name(fourth_line)? {
        None => {
            return Err(ExtractionError {
                line: fourth_line.to_string(),
                message: "Failed to parse second variable name!".to_string(),
            });
        }
        Some(second_variable) => second_variable,
    };

    Ok(InstructionInfo {
        assignment_name,
        first_variable,
        second_variable,
        operator: parse_operator(&operator)?,
    })
}

fn parse_operator(operator: &str) -> Result<Operator, ExtractionError> {
    match operator {
        "divided" => Ok(Operator::Div),
        "multiplied" => Ok(Operator::Mul),
        "added" => Ok(Operator::Add),
        "subtracted" => Ok(Operator::Sub),
        _ => Err(ExtractionError {
            line: "s".to_string(),
            message: format!("Unknown operator: {}", operator),
        }),
    }
}
