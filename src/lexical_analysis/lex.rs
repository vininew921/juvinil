use regex::Regex;

use crate::{
    error::{JuvinilError, JuvinilResult},
    lexical_analysis::token::Token,
};

use super::{comparators, jv_types, keyword, operators, regex_token, symbols};

pub fn tokenize(content: String) -> JuvinilResult<Vec<Token>> {
    let mut tokens: Vec<Token> = Vec::new();
    let special_symbols = "!;{}()[]";

    for (line_number, line_content) in content.lines().enumerate().into_iter() {
        if line_content.trim().is_empty() {
            continue;
        }

        let mut token_line: Vec<Token> = Vec::new();
        let mut processed_content: Vec<String> = Vec::new();
        let mut inside_string = false;
        let mut complete_string = String::new();

        for word in line_content.trim().split(" ") {
            if word.chars().next().unwrap() == '\"' {
                inside_string = true;
            }

            if inside_string {
                complete_string.push_str(word);

                if word.contains('\"') {
                    let splitted_word = word.split("\"");
                    inside_string = false;

                    for sw in splitted_word {
                        processed_content.push(sw.into());
                    }

                    continue;
                }
            }

            for sc in special_symbols.chars() {
                if word.contains(sc) {
                    for splitted_word in word.split(sc) {
                        if splitted_word.is_empty() {
                            processed_content.push(String::from(sc));
                            continue;
                        }

                        processed_content.push(splitted_word.into());
                    }
                }
            }

            processed_content.push(word.into());
        }

        if inside_string {
            return Err(JuvinilError::UnclosedString(line_number));
        }

        for lookahead in processed_content {
            token_line.push(process_token(lookahead.as_str(), line_number + 1)?);
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

    if let Some(symbol) = symbols::SYMBOLS.get(token) {
        return Ok(Token::from_symbol(symbol));
    }

    if let Some(comparator) = comparators::COMPARATORS.get(token) {
        return Ok(Token::from_comparator(comparator));
    }

    let regex_token = regex_token::REGEX_TOKEN_MAP
        .iter()
        .find(|op| Regex::new(op.regex_template).unwrap().is_match(token))
        .ok_or(JuvinilError::SyntaxError(String::from(token), line_number))?;

    Ok(Token::from_regex_token(regex_token, token))
}
