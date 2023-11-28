use regex::Regex;

use crate::{
    error::{JuvinilError, JuvinilResult},
    lexical_analysis::token::Token,
};

use super::{regex_token, token};

//Static function to tokenize the contents of a file
pub fn tokenize(content: String) -> JuvinilResult<Vec<Token>> {
    let mut tokens: Vec<Token> = Vec::new();

    //We enumerate each line of the file to output
    //better errors.
    for (line_number, line_content) in content.lines().enumerate() {
        //If the line is empty, just ignore it
        if line_content.trim().is_empty() {
            continue;
        }

        //The pre-process improves the "quality" of the line,
        //inserting spaces between special symbols and verifying if
        //there are any unclosed strings.
        let pre_processed_line = pre_process_line(line_content, line_number)?;

        let mut token_line: Vec<Token> = Vec::new();

        //Process each individual token
        //and put them inside the `token_line` vector
        for str_token in pre_processed_line {
            token_line.push(process_token(str_token.as_str(), line_number + 1)?);
        }

        tracing::info!("{} | {:?}", line_number + 1, token_line);

        //Add the current line to the full list of tokens
        tokens.extend(token_line);
    }

    //Push EOF as the final token to signal the end
    //of the file
    tokens.push(Token::eof(content.lines().count()));

    Ok(tokens)
}

//Verifies if there are any unclosed strings,
//build strings with spaces in a single String instance,
//and separates special symbols, putting spaces between them.
//For example, `if (teste < 2)` becomes `if ( teste < 2 )`
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
            if word.ends_with(&['\"'][..]) {
                inside_string = false;
                processed_content.push(complete_string[0..complete_string.len()].to_string());
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

//Maps the current &str against some static vectors (token.rs)
//to check if any of them match. For example,
//the &str `for` will be matched agaisn't the `token::KEYWORDS` vector,
//and a new token of type KEYWORD will be instantiated and returned.
//If the &str isn't matched agains't any of the static vectors, we
//check if it is a variable name (ID) or a String using REGEX
fn process_token(token: &str, line_number: usize) -> JuvinilResult<Token> {
    if let Some(keyword) = token::KEYWORDS.iter().find(|&x| *x == token) {
        return Ok(Token::new_keyword(String::from(*keyword), line_number));
    }

    if let Some(operator) = token::OPERATORS.iter().find(|&x| *x == token) {
        return Ok(Token::new_operator(String::from(*operator), line_number));
    }

    if let Some(jv_type) = token::JV_TYPES.iter().find(|&x| *x == token) {
        return Ok(Token::new_type(String::from(*jv_type), line_number));
    }

    if let Some(symbol) = token::SYMBOLS.iter().find(|&x| *x == token) {
        return Ok(Token::new_symbol(String::from(*symbol), line_number));
    }

    if let Some(comparator) = token::COMPARATORS.iter().find(|&x| *x == token) {
        return Ok(Token::new_comparator(
            String::from(*comparator),
            line_number,
        ));
    }

    let regex_token = regex_token::REGEX_TOKEN_MAP
        .iter()
        .find(|op| Regex::new(op.regex_template).unwrap().is_match(token))
        .ok_or(JuvinilError::LexicalError(String::from(token), line_number))?;

    Ok(Token::from_regex_token(regex_token, token, line_number))
}
