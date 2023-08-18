use std::fs;

pub mod error;
pub mod lex;

fn main() {
    let file_path = "src/test.jv";
    let file_result = fs::read_to_string(file_path);

    if let Ok(file) = file_result {
        if let Err(x) = lex::process_file_content(file) {
            println!("Error processing file contents: {}", x);
            return;
        }

        println!("Success!");
        return;
    }

    println!("Error opening file {file_path}");
}
