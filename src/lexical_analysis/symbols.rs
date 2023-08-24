use phf::phf_map;

#[derive(Debug, Clone, Copy)]
pub enum Symbol {
    ENDEXPR,
    POPEN,
    PCLOSE,
    BOPEN,
    BCLOSE,
    SBOPEN,
    SBCLOSE,
}

pub static SYMBOLS: phf::Map<&'static str, Symbol> = phf_map! {
    ";" => Symbol::ENDEXPR,
    "(" => Symbol::POPEN,
    ")" => Symbol::PCLOSE,
    "[" => Symbol::BOPEN,
    "]" => Symbol::BCLOSE,
    "{" => Symbol::SBOPEN,
    "}" => Symbol::SBCLOSE,
};
