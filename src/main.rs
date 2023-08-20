use std::fs;

use crate::lexical_analysis::lex;

pub mod error;
pub mod lexical_analysis;

fn main() {
    tracing_subscriber::fmt().pretty().init();

    let file_path = "src/test.jv";
    let file_result = fs::read_to_string(file_path);

    if let Ok(file) = file_result {
        if let Err(err) = lex::process_file_content(file) {
            tracing::error!("Error processing file contents: {}", err);
            return;
        }

        tracing::info!("Success!");
        return;
    }

    tracing::error!("Error opening file {file_path}");
}
