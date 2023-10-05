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

        //FIRST map
        let block_first = vec!["{"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let decls_first = vec!["int", "float", "boolean", "char", "string"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let decl_first = decls_first.clone();

        let stmts_first = vec![
            "{", "if", "while", "do", "break", "continue", "true", "false",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

        let stmt_first = stmts_first.clone();

        first.insert(LangInstructions::BLOCK, block_first);
        first.insert(LangInstructions::DECL, decl_first);
        first.insert(LangInstructions::STMT, stmt_first);

        Self {
            first,
            follow,
            lookahead,
        }
    }
}
