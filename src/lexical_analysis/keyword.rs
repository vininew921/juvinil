use phf::phf_map;

#[derive(Debug, Clone, Copy)]
pub enum Keyword {
    FUNC,
    RETURN,
    IF,
    ELSE,
    FOR,
    WHILE,
    BREAK,
    CONTINUE,
    SWITCH,
    CASE,
    TRUE,
    FALSE,
}

pub static KEYWORDS: phf::Map<&'static str, Keyword> = phf_map! {
    "func" => Keyword::FUNC,
    "return" => Keyword::RETURN,
    "if" => Keyword::IF,
    "else" => Keyword::ELSE,
    "for" => Keyword::FOR,
    "while" => Keyword::WHILE,
    "break" => Keyword::BREAK,
    "continue" => Keyword::CONTINUE,
    "switch" => Keyword::SWITCH,
    "case" => Keyword::CASE,
    "true" => Keyword::TRUE,
    "false" => Keyword::FALSE,
};
