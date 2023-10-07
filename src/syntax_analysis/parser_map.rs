use std::{collections::HashMap, fs};

use crate::{
    error::JuvinilResult,
    lexical_analysis::token::{Token, TokenType},
};

pub struct ParserMap {
    first: HashMap<Box<str>, Vec<String>>,
    follow: HashMap<Box<str>, Vec<String>>,
    lookahead: HashMap<Box<str>, Vec<String>>,
}

impl ParserMap {
    pub fn new() -> JuvinilResult<Self> {
        let lang_map = fs::read_to_string("lang.map")?;

        let mut first: HashMap<Box<str>, Vec<String>> = HashMap::new();
        let mut follow: HashMap<Box<str>, Vec<String>> = HashMap::new();
        let mut lookahead: HashMap<Box<str>, Vec<String>> = HashMap::new();

        let mut counter = -1;

        for line_content in lang_map.lines() {
            if line_content.starts_with("!--!") {
                counter += 1;
                continue;
            }

            let mut content = line_content.split("->");
            let key = content.next().unwrap().trim();
            let elements: Vec<String> = content
                .to_owned()
                .next()
                .unwrap()
                .split_whitespace()
                .map(|x| x.to_string())
                .collect();

            match counter {
                0 => first.insert(key.clone().into(), elements),
                1 => follow.insert(key.clone().into(), elements),
                2 => lookahead.insert(key.clone().into(), elements),
                _ => panic!("FOdeu"),
            };
        }

        Ok(Self {
            first,
            follow,
            lookahead,
        })
    }

    pub fn is_first(&self, entry_type: &str, token: &Token) -> bool {
        let mut str_value = token.value.clone();

        match token.token_type {
            TokenType::ID => str_value = "_ID_".to_string(),
            TokenType::NUMBER => str_value = "_NUM_".to_string(),
            _ => (),
        };

        if let Some(entry) = self.first.get(entry_type) {
            return entry.iter().any(|e| str_value.contains(e));
        }

        false
    }

    pub fn is_follow(&self, entry_type: &str, token: &Token) -> bool {
        let mut str_value = token.value.clone();

        match token.token_type {
            TokenType::ID => str_value = "_ID_".to_string(),
            TokenType::NUMBER => str_value = "_NUM_".to_string(),
            _ => (),
        };

        if let Some(entry) = self.follow.get(entry_type) {
            return entry.iter().any(|e| str_value.contains(e));
        }

        false
    }

    pub fn is_lookahead(&self, entry_type: &str, token: &Token) -> bool {
        let mut str_value = token.value.clone();

        match token.token_type {
            TokenType::ID => str_value = "_ID_".to_string(),
            TokenType::NUMBER => str_value = "_NUM_".to_string(),
            _ => (),
        };

        if let Some(entry) = self.lookahead.get(entry_type) {
            return entry.iter().any(|e| str_value.contains(e));
        }

        false
    }
}
