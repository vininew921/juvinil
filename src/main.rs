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
    use crate::lexical_analysis::{
        jv_types::JvType, keyword::Keyword, operators::Operator, token::Token,
    };

    use super::*;

    #[test]
    fn lex_basic_operators() {
        let file = fs::read_to_string("test_inputs/basic_operators.jv").unwrap();

        let tokens = lex::tokenize(file).unwrap();

        let float = Token::from_type(&JvType::FLOAT);
        let endexpr = Token::from_keyword(&Keyword::ENDEXPR);
        let assignment = Token::from_operator(&Operator::ASSIGNMENT);
        let sum = Token::from_operator(&Operator::ADDITION);
        let subtract = Token::from_operator(&Operator::SUBTRACTION);
        let divide = Token::from_operator(&Operator::DIVISION);
        let multiply = Token::from_operator(&Operator::MULTIPLICATION);
        let id_a = Token::from_id("a");
        let id_b = Token::from_id("b");
        let id_c = Token::from_id("c");
        let n1 = Token::from_number("1");
        let n4 = Token::from_number("4");

        let target = vec![
            float.clone(),
            id_a.clone(),
            endexpr.clone(),
            float.clone(),
            id_b.clone(),
            endexpr.clone(),
            float.clone(),
            id_c.clone(),
            endexpr.clone(),
            id_a.clone(),
            assignment.clone(),
            n4.clone(),
            endexpr.clone(),
            id_b.clone(),
            assignment.clone(),
            id_a.clone(),
            endexpr.clone(),
            id_c.clone(),
            assignment.clone(),
            id_a.clone(),
            sum.clone(),
            id_b.clone(),
            endexpr.clone(),
            id_c.clone(),
            assignment.clone(),
            id_a.clone(),
            subtract.clone(),
            id_b.clone(),
            endexpr.clone(),
            id_c.clone(),
            assignment.clone(),
            id_a.clone(),
            divide.clone(),
            id_b.clone(),
            endexpr.clone(),
            id_c.clone(),
            assignment.clone(),
            id_a.clone(),
            multiply.clone(),
            id_b.clone(),
            endexpr.clone(),
            id_c.clone(),
            assignment.clone(),
            id_c.clone(),
            sum.clone(),
            n1.clone(),
            endexpr.clone(),
        ];

        assert_eq!(format!("{:?}", tokens), format!("{:?}", target));
    }
}
