use phf::phf_map;

#[derive(Debug, Clone, Copy)]
pub enum Operator {
    ASSIGNMENT,
    ADDITION,
    SUBTRACTION,
    MULTIPLICATION,
    DIVISION,
    MODULE,
    NOT,
    BAND,
    BOR,
    BIGGERTHAN,
    SMALLERTHAN,
    EQORBIGGERTHAN,
    EQORSMALLERTHAN,
    EQ,
    DIFFERENT,
    AND,
    OR,
}

pub static OPERATORS: phf::Map<&'static str, Operator> = phf_map! {
    "=" => Operator::ASSIGNMENT,
    "+" => Operator::ADDITION,
    "-" => Operator::SUBTRACTION,
    "*" => Operator::MULTIPLICATION,
    "/" => Operator::DIVISION,
    "%" => Operator::MODULE,
    "!" => Operator::NOT,
    "&" => Operator::BAND,
    "|" => Operator::BOR,
    ">" => Operator::BIGGERTHAN,
    "<" => Operator::SMALLERTHAN,
    ">=" => Operator::EQORBIGGERTHAN,
    "<=" => Operator::EQORSMALLERTHAN,
    "==" => Operator::EQ,
    "!=" => Operator::DIFFERENT,
    "&&" => Operator::AND,
    "||" => Operator::OR,
};
