use regex::Regex;

use crate::{
    error::{JuvinilError, JuvinilResult},
    lexical_analysis::token::Token,
};

use super::{comparators, jv_types, keyword, operators, regex_token, symbols};

pub fn tokenize(content: String) -> JuvinilResult<Vec<Token>> {
    let mut tokens: Vec<Token> = Vec::new();

    for (line_number, line_content) in content.lines().enumerate().into_iter() {
        if line_content.trim().is_empty() {
            continue;
        }

        let pre_processed_line = pre_process_line(line_content, line_number)?;

        let mut token_line: Vec<Token> = Vec::new();

        for lookahead in pre_processed_line {
            token_line.push(process_token(lookahead.as_str(), line_number + 1)?);
        }

        tracing::info!("{} | {:?}", line_number + 1, token_line);
        tokens.extend(token_line);
    }

    tokens.push(Token::eof());

    Ok(tokens)
}

fn pre_process_line(line_content: &str, line_number: usize) -> JuvinilResult<Vec<String>> {
    let special_symbols = vec![
        "!", ";", "++", "--", "+=", "-=", "{", "}", "(", ")", "[", "]",
    ];

    if line_content.chars().filter(|c| c == &'\"').count() % 2 != 0 {
        return Err(JuvinilError::UnclosedString(line_number + 1));
    }

    let mut processed_content: Vec<String> = Vec::new();
    let mut inside_string = false;
    let mut complete_string = String::new();

    for word in line_content.trim().split(" ") {
        let mut has_special_character = false;

        if word.chars().next().unwrap() == '\"' {
            inside_string = true;
        }

        //If inside a string, we append the `complete_string` variable until we find another <">
        if inside_string {
            for spw in word.split_inclusive("\"") {
                if spw.chars().last().unwrap() == '\"' {
                    if complete_string.is_empty() {
                        complete_string.push_str(spw);
                        continue;
                    }

                    inside_string = false;
                    complete_string.push_str(spw);
                    processed_content.push(complete_string.clone());
                    complete_string.clear();
                    continue;
                }

                if inside_string {
                    complete_string.push_str(spw);
                    complete_string.push_str(" ");
                    continue;
                }

                processed_content.push(spw.into());
            }

            continue;
        }

        //If a word contains a special character, we split it from the word to process it
        //separately. We're already sure that the word isn't a string here
        //To do: improve this, cases like i++) won't work, or count++;
        for sc in special_symbols.clone() {
            if word.trim().contains(sc) && word.trim().len() > 1 {
                has_special_character = true;
                for splitted_word in word.split(sc) {
                    if splitted_word.is_empty() {
                        processed_content.push(sc.into());
                        continue;
                    }

                    processed_content.push(splitted_word.into());
                }
            }
        }

        if !has_special_character {
            processed_content.push(word.into());
        }
    }

    if inside_string {
        return Err(JuvinilError::UnclosedString(line_number));
    }

    Ok(processed_content)
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
        .ok_or(JuvinilError::LexicalError(String::from(token), line_number))?;

    Ok(Token::from_regex_token(regex_token, token))
}
