use std::{env, vec};

use tests::rhyming_test::is_aabb_rhyming_scheme;

mod generic_parser;
mod instruction_parser;
mod struct_parser;
mod structs;
mod tests;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.is_empty() {
        println!("Please provide a file path!");
        return;
    }

    let file_path = &args[1];

    let file_content = match std::fs::read_to_string(file_path) {
        Ok(file_content) => file_content,
        Err(err) => {
            println!("Failed to read file: {}", err);
            return;
        }
    };

    let sections = file_content.split("###");

    let sections: Vec<&str> = sections.collect();

    println!("{:?}", sections);

    for section in sections.iter() {
        let is_rhyming = is_aabb_rhyming_scheme(section);

        if !is_rhyming {
            println!("The code does not have a AABB rhyming scheme!");
            return;
        }
    }

    let mut struct_block: Vec<String> = vec![];

    let struct_section = match sections.get(0) {
        None => {
            println!("The code does not have a struct block!");
            return;
        }
        Some(first_section) => first_section,
    };

    let struct_infos = struct_section.split("\n\n");

    let struct_infos: Vec<&str> = struct_infos.collect();

    for struct_info in struct_infos.iter() {
        let struct_info = struct_parser::parse_struct(struct_info);

        let struct_info = match struct_info {
            Err(err) => {
                println!("Failed to parse struct: {:?}", err);
                return;
            }
            Ok(struct_info) => struct_info,
        };

        struct_block.push(struct_info.to_string());
    }

    let mut instruction_block: Vec<String> = vec![];

    let instruction_section = match sections.get(1) {
        None => {
            println!("The code does not have an instruction block!");
            return;
        }
        Some(second_section) => second_section,
    };

    let instruction_infos = instruction_section.split("\n\n");

    let instruction_infos: Vec<&str> = instruction_infos.collect();

    for instruction_info in instruction_infos.iter() {
        let instruction_info = instruction_parser::parse_instruction(instruction_info);

        let instruction_info = match instruction_info {
            Err(err) => {
                println!("Failed to parse instruction: {:?}", err);
                return;
            }
            Ok(instruction_info) => instruction_info,
        };

        instruction_block.push(instruction_info.to_string());
    }

    //Construct the final code

    let mut final_code = String::new();

    for struct_info in struct_block.iter() {
        final_code.push_str(struct_info);
        final_code.push_str("\n");
    }

    final_code.push_str("\n");

    for instruction_info in instruction_block.iter() {
        final_code.push_str(instruction_info);
        final_code.push_str("\n");
    }

    println!("{}", final_code);
}
