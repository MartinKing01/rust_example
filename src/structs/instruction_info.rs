use std::fmt::Display;

use super::operator::Operator;

#[derive(Debug)]
pub struct InstructionInfo {
    pub assignment_name: Option<String>,
    pub first_variable: String,
    pub second_variable: String,
    pub operator: Operator,
}

impl Display for InstructionInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let assignment_name = match &self.assignment_name {
            None => "".to_string(),
            Some(assignment_name) => assignment_name.to_string(),
        };

        write!(
            f,
            "{} = {} {} {}",
            assignment_name, self.first_variable, self.operator, self.second_variable
        )
    }
}
