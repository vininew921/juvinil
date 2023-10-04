use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum LangInstructions {
    DECL,
    STMT,
    ASGN,
    BLOCK,
    JVTYPE,
    FUNC,
    PARAMS,
    FUNCDECL,
    PARAMDECL,
    BOOLEXPR,
    JOIN,
    EQUALITY,
    CMP,
    EXPR,
    BRN,
    TERM,
    UNIT,
    FACTOR,
}

pub struct ParserMap {
    first: HashMap<LangInstructions, Vec<String>>,
    follow: HashMap<LangInstructions, Vec<String>>,
    lookahead: HashMap<LangInstructions, Vec<String>>,
}

impl ParserMap {
    pub fn new() -> Self {
        let mut first: HashMap<LangInstructions, Vec<String>> = HashMap::new();
        let mut follow: HashMap<LangInstructions, Vec<String>> = HashMap::new();
        let mut lookahead: HashMap<LangInstructions, Vec<String>> = HashMap::new();

        first.insert(LangInstructions::DECL, vec!["decl".into()]);
        first.insert(LangInstructions::FUNC, vec!["func".into()]);

        Self {
            first,
            follow,
            lookahead,
        }
    }
}
