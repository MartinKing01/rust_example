use regex::{Captures, Regex};

use crate::{
    struct_parser::varifable_type_to_rust_type,
    structs::{instruction_info::InstructionInfo, param_info::ParamInfo},
};

pub fn extract_from_pattern<'a>(
    line: &'a str,
    pattern: &'a str,
) -> Result<Option<Captures<'a>>, ExtractionError> {
    let pattern = Regex::new(pattern);

    let pattern = match pattern {
        Ok(pattern) => pattern,
        Err(err) => {
            return Err(ExtractionError {
                line: line.to_string(),
                message: err.to_string(),
            });
        }
    };

    match pattern.captures(line) {
        None => {
            return Ok(None);
        }
        Some(captures) => {
            return Ok(Some(captures));
        }
    };
}

pub fn extract_variable_name(line: &str) -> Result<Option<String>, ExtractionError> {
    let variables_name_captures = extract_from_pattern(line, r"\*(.*)\*");

    match variables_name_captures {
        Ok(variables_name_captures) => {
            let variables_name_captures = match variables_name_captures {
                None => {
                    return Ok(None);
                }
                Some(variables_name_captures) => variables_name_captures,
            };

            let variable_name = match variables_name_captures.get(1) {
                None => {
                    return Ok(None);
                }
                Some(variable_name) => variable_name.as_str(),
            };

            Ok(Some(variable_name.to_string()))
        }
        Err(err) => Err(err),
    }
}

pub fn extract_operator(line: &str) -> Result<Option<String>, ExtractionError> {
    let operator_captures = extract_from_pattern(line, r"\*\*\*(.*)\*\*\*");

    match operator_captures {
        Ok(operator_captures) => {
            let operator_captures = match operator_captures {
                None => {
                    return Ok(None);
                }
                Some(operator_captures) => operator_captures,
            };

            let operator = match operator_captures.get(1) {
                None => {
                    return Ok(None);
                }
                Some(operator) => operator.as_str(),
            };

            Ok(Some(operator.to_string()))
        }
        Err(err) => Err(err),
    }
}

pub fn extract_name(line: &str) -> Result<Option<String>, ExtractionError> {
    let captures = extract_from_pattern(line, r"\*\*(.*)\*\*");

    match captures {
        Ok(captures) => {
            let captures = match captures {
                None => {
                    return Ok(None);
                }
                Some(captures) => captures,
            };

            let name = match captures.get(1) {
                None => {
                    return Ok(None);
                }
                Some(name) => name.as_str(),
            };

            Ok(Some(name.to_string()))
        }
        Err(err) => Err(err),
    }
}

pub fn extract_param(line: &str) -> Result<Option<ParamInfo>, ExtractionError> {
    let name_captures = extract_from_pattern(line, r"\*(.*)\*");
    let param_type_captures = extract_from_pattern(line, r"_(.*)_");

    let name = match name_captures {
        Ok(name_captures) => {
            let name_captures = match name_captures {
                None => {
                    return Ok(None);
                }
                Some(name_captures) => name_captures,
            };

            let name = match name_captures.get(1) {
                None => {
                    return Ok(None);
                }
                Some(name) => name.as_str(),
            };

            Ok(Some(name.to_string()))
        }
        Err(err) => Err(err),
    };

    let param_type = match param_type_captures {
        Ok(param_type_captures) => {
            let param_type_captures = match param_type_captures {
                None => {
                    return Ok(None);
                }
                Some(param_type_captures) => param_type_captures,
            };

            let param_type = match param_type_captures.get(1) {
                None => {
                    return Ok(None);
                }
                Some(param_type) => param_type.as_str(),
            };

            Ok(Some(param_type.to_string()))
        }
        Err(err) => Err(err),
    };

    match (name, param_type) {
        (Ok(name), Ok(param_type)) => {
            let name = match name {
                None => {
                    return Ok(None);
                }
                Some(name) => name,
            };
            let param_type = match param_type {
                None => {
                    return Ok(None);
                }
                Some(param_type) => param_type,
            };

            let param_type = varifable_type_to_rust_type(&param_type);

            let param_type = match param_type {
                Ok(param_type) => param_type,
                Err(err) => {
                    return Err(ExtractionError {
                        line: line.to_string(),
                        message: err.to_string(),
                    });
                }
            };

            Ok(Some(ParamInfo {
                name: name,
                param_type: param_type,
            }))
        }
        (Err(err), _) => Err(err),
        (_, Err(err)) => Err(err),
    }
}

#[derive(Debug)]
pub struct ExtractionError {
    pub line: String,
    pub message: String,
}
