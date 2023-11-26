use regex::Regex;

use crate::{
    error::{JuvinilError, JuvinilResult},
    lexical_analysis::token::Token,
};

use super::{regex_token, token};

pub fn tokenize(content: String) -> JuvinilResult<Vec<Token>> {
    let mut tokens: Vec<Token> = Vec::new();

    for (line_number, line_content) in content.lines().enumerate() {
        if line_content.trim().is_empty() {
            continue;
        }

        let pre_processed_line = pre_process_line(line_content, line_number)?;

        let mut token_line: Vec<Token> = Vec::new();

        for str_token in pre_processed_line {
            token_line.push(process_token(str_token.as_str(), line_number + 1)?);
        }

        tracing::info!("{} | {:?}", line_number + 1, token_line);
        tokens.extend(token_line);
    }

    tokens.push(Token::eof());

    Ok(tokens)
}

fn pre_process_line(line_content: &str, line_number: usize) -> JuvinilResult<Vec<String>> {
    if line_content.chars().filter(|c| c == &'\"').count() % 2 != 0 {
        return Err(JuvinilError::UnclosedString(line_number + 1));
    }

    let mut processed_content: Vec<String> = Vec::new();
    let mut inside_string = false;
    let mut complete_string = String::new();

    for word in line_content.split_whitespace() {
        if word.starts_with('\"') {
            inside_string = true;
        }

        //If inside a string, we append the `complete_string` variable until we find a <";>
        if inside_string {
            complete_string.push_str(word);
            if word.ends_with(&['\"', ';'][..]) {
                inside_string = false;
                processed_content.push(complete_string[0..complete_string.len() - 1].to_string());
                processed_content.push(";".into());
                complete_string.clear();
            } else {
                complete_string.push(' ');
            }

            continue;
        }

        let special_symbols = &[
            "++", "--", "+=", "-=", "{", "}", "(", ")", "[", "]", ";", ",",
        ];

        let mut has_special_symbol = false;
        for special_symbol in special_symbols {
            if word.contains(special_symbol) {
                has_special_symbol = true;
                let mut pushed_special = false;
                for part in word.split(special_symbol) {
                    if !part.is_empty() {
                        processed_content.push(part.to_string());
                    } else if !pushed_special {
                        processed_content.push(special_symbol.to_string());
                        pushed_special = true;
                    }
                }
            }
        }

        if !has_special_symbol {
            processed_content.push(word.to_string());
        }
    }

    if inside_string {
        return Err(JuvinilError::UnclosedString(line_number));
    }

    Ok(processed_content)
}

fn process_token(token: &str, line_number: usize) -> JuvinilResult<Token> {
    if let Some(keyword) = token::KEYWORDS.iter().find(|&x| *x == token) {
        return Ok(Token::new_keyword(String::from(*keyword)));
    }

    if let Some(comparator) = token::COMPARATORS.iter().find(|&x| *x == token) {
        return Ok(Token::new_comparator(String::from(*comparator)));
    }

    if let Some(operator) = token::OPERATORS.iter().find(|&x| *x == token) {
        return Ok(Token::new_operator(String::from(*operator)));
    }

    if let Some(jv_type) = token::JV_TYPES.iter().find(|&x| *x == token) {
        return Ok(Token::new_type(String::from(*jv_type)));
    }

    if let Some(symbol) = token::SYMBOLS.iter().find(|&x| *x == token) {
        return Ok(Token::new_symbol(String::from(*symbol)));
    }

    let regex_token = regex_token::REGEX_TOKEN_MAP
        .iter()
        .find(|op| Regex::new(op.regex_template).unwrap().is_match(token))
        .ok_or(JuvinilError::LexicalError(String::from(token), line_number))?;

    Ok(Token::from_regex_token(regex_token, token))
}
