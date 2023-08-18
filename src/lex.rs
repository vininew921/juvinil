use regex::Regex;

use crate::error::{JuvinilError, JuvinilResult};

pub fn process_file_content(content: String) -> JuvinilResult<()> {
    println!("File content: ");
    println!("{}", content);

    let lines = content.lines();

    let mut tokens: Vec<Vec<String>> = Vec::new();

    for (line_number, line_content) in lines.enumerate().into_iter() {
        if line_content.trim() == "" {
            continue;
        }

        let mut token_line_vec: Vec<String> = Vec::new();
        for token in line_content.split(" ") {
            token_line_vec.push(process_token(String::from(token), line_number + 1)?);
        }

        tokens.push(token_line_vec.clone());
        println!("{} - {:?}", line_number + 1, token_line_vec);
    }

    Ok(())
}

fn process_token(token: String, line_number: usize) -> JuvinilResult<String> {
    //Token definition
    let token_keyword_init = "<keyword init>";
    let token_op = "<op %s>";
    let token_int = "<int %s>";
    let token_string = "<string %s>";
    let token_id = "<id %s>";

    let op_map: Vec<RegexToken> = vec![
        RegexToken::new("[+\\-*%/=^&|!]", token_op, false),
        RegexToken::new("^\\d+$", token_int, false),
        RegexToken::new(r#"^"[^"]*"$"#, token_string, true),
        RegexToken::new("^[A-Za-z]+", token_id, false),
    ];

    if token == "init" {
        return Ok(String::from(token_keyword_init));
    }

    let regex_token = op_map
        .iter()
        .find(|op| op.regex.is_match(token.as_str()))
        .ok_or(JuvinilError::NoOptionError(token.clone(), line_number))?;

    Ok(regex_token.format_result(token))
}

struct RegexToken {
    pub regex: Regex,
    pub result: String,
    pub remove_quotes: bool,
}

impl RegexToken {
    pub fn new(regex: &str, result: &str, remove_quotes: bool) -> Self {
        Self {
            regex: Regex::new(regex).unwrap(),
            result: String::from(result),
            remove_quotes,
        }
    }

    pub fn format_result(&self, token: String) -> String {
        if self.remove_quotes {
            return self
                .result
                .clone()
                .replace("%s", token.as_str())
                .replace("\"", "");
        }

        self.result.clone().replace("%s", token.as_str())
    }
}
