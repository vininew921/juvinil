use std::{collections::HashMap, fs};

use crate::error::JuvinilResult;

pub struct ParserMap {
    pub first: HashMap<String, Vec<String>>,
    pub follow: HashMap<String, Vec<String>>,
    pub lookahead: HashMap<String, Vec<String>>,
}

impl ParserMap {
    pub fn new() -> JuvinilResult<Self> {
        let lang_map = fs::read_to_string("lang.map")?;

        let mut first: HashMap<String, Vec<String>> = HashMap::new();
        let mut follow: HashMap<String, Vec<String>> = HashMap::new();
        let mut lookahead: HashMap<String, Vec<String>> = HashMap::new();

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
                0 => first.insert(key.into(), elements),
                1 => follow.insert(key.into(), elements),
                2 => lookahead.insert(key.into(), elements),
                _ => panic!("FOdeu"),
            };
        }

        for value in first.clone() {
            tracing::info!("First: {:?}", value);
        }

        for value in follow.clone() {
            tracing::info!("Follow: {:?}", value);
        }

        for value in lookahead.clone() {
            tracing::info!("Lookahead: {:?}", value);
        }

        Ok(Self {
            first,
            follow,
            lookahead,
        })
    }
}
