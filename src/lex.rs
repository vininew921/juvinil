use regex::Regex;

use crate::{
    error::{JuvinilError, JuvinilResult},
    token_map,
};

pub fn process_file_content(content: String) -> JuvinilResult<()> {
    tracing::info!("File content: \n{}", content);

    let mut tokens: Vec<Vec<String>> = Vec::new();

    for (line_number, line_content) in content.lines().enumerate().into_iter() {
        if line_content.trim() == "" {
            continue;
        }

        let mut token_line_vec: Vec<String> = Vec::new();
        for token in line_content.split(" ") {
            token_line_vec.push(process_token(token, line_number + 1)?);
        }

        tokens.push(token_line_vec.clone());
        tracing::info!("{} - {:?}", line_number + 1, token_line_vec);
    }

    Ok(())
}

fn process_token(token: &str, line_number: usize) -> JuvinilResult<String> {
    if token == "init" {
        return Ok(String::from(token_map::T_KEYWORD_INIT));
    }

    let regex_token = token_map::TOKEN_MAP
        .iter()
        .find(|op| Regex::new(op.regex_template).unwrap().is_match(token))
        .ok_or(JuvinilError::NoRegexMatch(String::from(token), line_number))?;

    Ok(regex_token.format_token(token))
}
