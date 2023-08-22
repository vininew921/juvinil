use std::fs;

use error::JuvinilResult;

use crate::lexical_analysis::lex;

pub mod error;
pub mod lexical_analysis;

fn main() {
    tracing_subscriber::fmt().pretty().init();

    if let Err(err) = run("src/test.jv") {
        tracing::error!("{}", err);
        std::process::exit(1);
    }
}

fn run(file_path: &str) -> JuvinilResult<()> {
    let file = fs::read_to_string(file_path)?;
    tracing::info!("Successfully read contents of file {}", file_path);

    let _tokens = lex::tokenize(file)?;
    tracing::info!("Successfully tokenized file contents");

    Ok(())
}
