use phf::phf_map;

#[derive(Debug, Clone, Copy)]
pub enum Keyword {
    FUNC,
    RETURN,
    IF,
    ELSE,
    FOR,
    WHILE,
    SWITCH,
    CASE,
    TRUE,
    FALSE,
    ENDEXPR,
}

pub static KEYWORDS: phf::Map<&'static str, Keyword> = phf_map! {
    "func" => Keyword::FUNC,
    "return" => Keyword::RETURN,
    "if" => Keyword::IF,
    "else" => Keyword::ELSE,
    "for" => Keyword::FOR,
    "while" => Keyword::WHILE,
    "switch" => Keyword::SWITCH,
    "case" => Keyword::CASE,
    "true" => Keyword::TRUE,
    "false" => Keyword::FALSE,
    ";" => Keyword::ENDEXPR,
};
