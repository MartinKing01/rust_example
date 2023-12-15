#[cfg(test)]
mod tests {
    use crate::{instruction_parser::parse_instruction, struct_parser::parse_struct};

    #[test]
    fn test_instruction_parsing_1() {
        let input = "A *mouse* begins his journey in the land,
        A *dog* begins his journey in the land,
        A ***divided*** begins his journey in the land,
        A *cat* begins his journey in the land";

        match parse_instruction(input) {
            Ok(instruction_info) => {
                println!("{}", instruction_info);
            }
            Err(error) => {
                panic!("Failed to parse struct: {:?}", error);
            }
        }
    }
}
