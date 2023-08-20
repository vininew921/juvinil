use regex::Regex;

use crate::{
    error::{JuvinilError, JuvinilResult},
    lexical_analysis::token::Token,
};

use super::{jv_types, keyword, operators, regex_token};

pub fn process_file_content(content: String) -> JuvinilResult<()> {
    tracing::info!("File content: \n{}", content);

    let mut tokens: Vec<Vec<Token>> = Vec::new();

    for (line_number, line_content) in content.lines().enumerate().into_iter() {
        if line_content.trim().is_empty() {
            continue;
        }

        let mut token_line: Vec<Token> = Vec::new();

        for token in line_content.split(" ") {
            let stripped_token = token.replace(";", "");
            token_line.push(process_token(stripped_token.as_str(), line_number + 1)?);
        }

        tokens.push(token_line.clone());
        tracing::info!("{} - {:?}", line_number + 1, token_line);
    }

    Ok(())
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
        .ok_or(JuvinilError::NoRegexMatch(String::from(token), line_number))?;

    Ok(Token::from_regex_token(regex_token, token))
}
