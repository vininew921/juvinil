use regex::Regex;

use crate::{
    error::{JuvinilError, JuvinilResult},
    lexical_analysis::token::Token,
};

use super::{jv_types, keyword, operators, regex_token};

pub fn tokenize(content: String) -> JuvinilResult<Vec<Token>> {
    let mut tokens: Vec<Token> = Vec::new();

    for (line_number, line_content) in content.lines().enumerate().into_iter() {
        if line_content.trim().is_empty() {
            continue;
        }

        let mut token_line: Vec<Token> = Vec::new();

        for lookahead in line_content.trim().split(" ") {
            if lookahead.chars().last().unwrap() == ';' {
                token_line.push(process_token(
                    lookahead.split(";").next().unwrap(),
                    line_number + 1,
                )?);

                token_line.push(process_token(";", line_number + 1)?);
            } else {
                token_line.push(process_token(lookahead, line_number + 1)?);
            }
        }

        tracing::info!("{} | {:?}", line_number + 1, token_line);
        tokens.extend(token_line);
    }

    Ok(tokens)
}

fn process_token(token: &str, line_number: usize) -> JuvinilResult<Token> {
    if let Some(keyword) = keyword::KEYWORDS.get(token) {
        return Ok(Token::from_keyword(keyword));
    }

    if let Some(operator) = operators::OPERATORS.get(token) {
        return Ok(Token::from_operator(operator));
    }

    if let Some(jv_type) = jv_types::JV_TYPES.get(token) {
        return Ok(Token::from_type(jv_type));
    }

    let regex_token = regex_token::REGEX_TOKEN_MAP
        .iter()
        .find(|op| Regex::new(op.regex_template).unwrap().is_match(token))
        .ok_or(JuvinilError::SyntaxError(String::from(token), line_number))?;

    Ok(Token::from_regex_token(regex_token, token))
}
