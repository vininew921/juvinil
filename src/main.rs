use std::fs;

use error::JuvinilResult;

use crate::lexical_analysis::lex;

pub mod error;
pub mod lexical_analysis;

fn main() {
    tracing_subscriber::fmt().pretty().init();

    if let Err(err) = run("test_inputs/test.jv") {
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
    use crate::lexical_analysis::{operators::Operator, token::Token};

    use super::*;

    #[test]
    fn lex_operators() {
        let file_content = fs::read_to_string("test_inputs/operators.jv").unwrap();

        let a = Token::from_id("a");
        let n1 = Token::from_number("1");
        let asgn = Token::from_operator(&Operator::ASSIGN);
        let sum = Token::from_operator(&Operator::ADD);
        let sub = Token::from_operator(&Operator::SUBTRACT);
        let mul = Token::from_operator(&Operator::MULTIPLY);
        let div = Token::from_operator(&Operator::DIVIDE);
        let module = Token::from_operator(&Operator::MODULE);
        let not = Token::from_operator(&Operator::NOT);
        let band = Token::from_operator(&Operator::BINARYAND);
        let bor = Token::from_operator(&Operator::BINARYOR);

        let tokens = lex::tokenize(file_content).unwrap();

        let target = vec![
            a.clone(),
            asgn.clone(),
            n1.clone(),
            a.clone(),
            sum.clone(),
            n1.clone(),
            a.clone(),
            sub.clone(),
            n1.clone(),
            a.clone(),
            mul.clone(),
            n1.clone(),
            a.clone(),
            div.clone(),
            n1.clone(),
            a.clone(),
            module.clone(),
            n1.clone(),
            not.clone(),
            a.clone(),
            a.clone(),
            band.clone(),
            n1.clone(),
            a.clone(),
            bor.clone(),
            n1.clone(),
        ];

        assert_eq!(format!("{:?}", tokens), format!("{:?}", target));
    }
}
