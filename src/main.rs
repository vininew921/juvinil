use std::fs;

use error::JuvinilResult;

use crate::lexical_analysis::lex;

pub mod error;
pub mod lexical_analysis;

fn main() {
    tracing_subscriber::fmt().pretty().init();

    if let Err(err) = run("test_inputs/for.jv") {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_operators_ok() {
        let file_content = fs::read_to_string("test_inputs/operators.jv").unwrap();

        let tokens = lex::tokenize(file_content);

        assert!(tokens.is_ok(), "Should be OK");
    }

    #[test]
    fn types_ok() {
        let file_content = fs::read_to_string("test_inputs/types.jv").unwrap();

        let tokens = lex::tokenize(file_content);

        assert!(tokens.is_ok(), "Should be OK");
    }

    #[test]
    fn strings_ok() {
        let file_content = fs::read_to_string("test_inputs/strings.jv").unwrap();

        let tokens = lex::tokenize(file_content);

        assert!(tokens.is_ok(), "Should be OK");
    }

    #[test]
    fn strings_error() {
        let file_content = fs::read_to_string("test_inputs/strings_error.jv").unwrap();

        let tokens = lex::tokenize(file_content);

        assert!(tokens.is_err(), "Should be ERR");
    }

    #[test]
    fn if_ok() {
        let file_content = fs::read_to_string("test_inputs/if.jv").unwrap();

        let tokens = lex::tokenize(file_content);

        assert!(tokens.is_ok(), "Should be OK");
    }

    #[test]
    fn for_ok() {
        let file_content = fs::read_to_string("test_inputs/for.jv").unwrap();

        let tokens = lex::tokenize(file_content);

        assert!(tokens.is_ok(), "Should be OK");
    }
}
