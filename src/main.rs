use std::{env, fs, process::Command};

use juvinil::{error::JuvinilResult, lexical_analysis::lex, syntax_analysis::parser::Parser};

fn main() {
    //Initializes logging
    tracing_subscriber::fmt().pretty().init();

    //Call the `run` function, returing an
    //reporting an error in case one occurs
    if let Err(err) = run("test_inputs/test.jv") {
        tracing::error!("{}", err);
        std::process::exit(1);
    }
}

//Run all steps of the compiler
fn run(file_path: &str) -> JuvinilResult<()> {
    //Start by reading the given file into a String
    tracing::info!("--------READING INPUT--------");
    let file = fs::read_to_string(file_path)?;
    tracing::info!("Successfully read contents of file {}", file_path);

    //Take the current file and tokenize it (lex.rs)
    tracing::info!("--------LEXICAL ANALYSIS--------");
    let tokens = lex::tokenize(file)?;
    tracing::info!("Successfully tokenized file contents");

    //Take the resulting tokens and parse them,
    //which verifies the code syntax and also
    //builds the intermediary code
    tracing::info!("--------SYNTAX ANALYSIS--------");
    let mut parser = Parser::new(tokens)?;
    parser.parse()?;
    tracing::info!("Successfully parsed file contents");

    //Take the intermediary code from the parser
    //and dump it into a `.c` file
    tracing::info!("--------DUMPING INTERMEDIARY CODE--------");
    parser.dump_intermediary_code("compiler_results/result.cpp")?;
    tracing::info!("Successfully dumped intermediary code");

    tracing::info!("--------EXECUTING INTERMEDIARY CODE--------");
    //Run gcc to compile the generated code
    Command::new("g++.exe")
        .arg("-o")
        .arg("compiler_results/result.exe")
        .arg("compiler_results/result.cpp")
        .status()
        .unwrap();

    //Run the generated .exe
    Command::new("compiler_results/result.exe").spawn().unwrap();

    Ok(())
}

//Just some random tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_operators_ok() {
        let file_content = fs::read_to_string("test_inputs/operators.jv").unwrap();

        let tokens = lex::tokenize(file_content);

        assert!(tokens.is_ok(), "Should be OK");

        let mut parser = Parser::new(tokens.unwrap()).unwrap();
        let result = parser.parse();

        assert!(result.is_ok(), "Should be OK");
    }

    #[test]
    fn types_ok() {
        let file_content = fs::read_to_string("test_inputs/types.jv").unwrap();

        let tokens = lex::tokenize(file_content);

        assert!(tokens.is_ok(), "Should be OK");

        let mut parser = Parser::new(tokens.unwrap()).unwrap();
        let result = parser.parse();

        assert!(result.is_ok(), "Should be OK");
    }

    #[test]
    fn strings_ok() {
        let file_content = fs::read_to_string("test_inputs/strings.jv").unwrap();

        let tokens = lex::tokenize(file_content);

        assert!(tokens.is_ok(), "Should be OK");

        let mut parser = Parser::new(tokens.unwrap()).unwrap();
        let result = parser.parse();

        assert!(result.is_ok(), "Should be OK");
    }

    #[test]
    fn if_ok() {
        let file_content = fs::read_to_string("test_inputs/if.jv").unwrap();

        let tokens = lex::tokenize(file_content);

        assert!(tokens.is_ok(), "Should be OK");

        let mut parser = Parser::new(tokens.unwrap()).unwrap();
        let result = parser.parse();

        assert!(result.is_ok(), "Should be OK");
    }

    #[test]
    fn for_ok() {
        let file_content = fs::read_to_string("test_inputs/for.jv").unwrap();

        let tokens = lex::tokenize(file_content);

        assert!(tokens.is_ok(), "Should be OK");

        let mut parser = Parser::new(tokens.unwrap()).unwrap();
        let result = parser.parse();

        assert!(result.is_ok(), "Should be OK");
    }
}
