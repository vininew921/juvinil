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
    _map: HashMap<LangInstructions, Vec<String>>,
}

impl ParserMap {
    pub fn new() -> Self {
        let mut map: HashMap<LangInstructions, Vec<String>> = HashMap::new();

        map.insert(LangInstructions::DECL, vec!["decl".into()]);
        map.insert(LangInstructions::FUNC, vec!["func".into()]);

        Self { _map: map }
    }
}
