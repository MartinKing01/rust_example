#[cfg(test)]
mod tests {
    use crate::struct_parser::parse_struct;

    #[test]
    fn test_struct_parsing_1() {
        let input = "A **Hero** begins his journey in the land,
    his *name* is _wordly_, as I understand,
    at the unknown *age*, but _countable_,
    he should sit now at the table.";

        match parse_struct(input) {
            Ok(struct_info) => {
                println!("struct {} {{", struct_info.name);
                for (field_name, field_type) in struct_info.fields.iter() {
                    println!("    {}: {},", field_name, field_type);
                }
                println!("}}");
            }
            Err(error) => {
                panic!("Failed to parse struct: {:?}", error);
            }
        }
    }

    #[test]
    fn test_struct_parsing_2() {
        let input = r"The **Enemy** is a creature of the dark,
        his *health* is _countable_, but not a mark.";

        match parse_struct(input) {
            Ok(struct_info) => {
                println!("struct {} {{", struct_info.name);
                for (field_name, field_type) in struct_info.fields.iter() {
                    println!("    {}: {},", field_name, field_type);
                }
                println!("}}");
            }
            Err(error) => {
                panic!("Failed to parse struct: {:?}", error);
            }
        }
    }
}
