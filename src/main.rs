mod tokenizer;
mod token;

use std::{env, path::Path};

use tokenizer::tokenize;

fn main() {
    // Get args from command line
    let args: Vec<String> = env::args().collect();
    // Check for the input file
    if args.len() != 2 {
       println!("Usage: <compiler> file.m4z");
       return; 
    }

    let input_file_path = Path::new(&args[1]).to_owned();
    // Check for input file extension
    match input_file_path.to_str().to_owned().expect("").find(".m4z") {
        Some(_) => {},
        None => {
            println!("No m4z file extension found.");
            return;
        }
    }
    // Check for file path existence
    if !input_file_path.exists() {
        println!("The m4z file path doesn't exist.");
        return;
    }

    let input_file_path: String = input_file_path.to_str().expect("").to_owned();
    let mut input_file_buffer = std::fs::read_to_string(input_file_path).expect("Can't read the file");
    let tokens_test = tokenize(&mut input_file_buffer);
    
    dbg!(tokens_test);
}
