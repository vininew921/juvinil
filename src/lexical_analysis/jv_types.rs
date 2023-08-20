use phf::phf_map;

#[derive(Debug, Clone, Copy)]
pub enum JvType {
    VOID,
    INT,
    FLOAT,
    BOOLEAN,
    CHAR,
    STRING,
}

pub static JV_TYPES: phf::Map<&'static str, JvType> = phf_map! {
    "void" => JvType::VOID,
    "int" => JvType::INT,
    "float" => JvType::FLOAT,
    "boolean" => JvType::BOOLEAN,
    "char" => JvType::CHAR,
    "string" => JvType::STRING
};
