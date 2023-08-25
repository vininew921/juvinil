use phf::phf_map;

#[derive(Debug, Clone, Copy)]
pub enum Operator {
    ASSIGN,
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
    MODULE,
    NOT,
    BINARYAND,
    BINARYOR,
    INCREMENT,
    DECREMENT,
    INCREMENTBY,
    DECREMENTBY,
}

pub static OPERATORS: phf::Map<&'static str, Operator> = phf_map! {
    "=" => Operator::ASSIGN,
    "+" => Operator::ADD,
    "-" => Operator::SUBTRACT,
    "*" => Operator::MULTIPLY,
    "/" => Operator::DIVIDE,
    "%" => Operator::MODULE,
    "!" => Operator::NOT,
    "&" => Operator::BINARYAND,
    "|" => Operator::BINARYOR,
    "++" => Operator::INCREMENT,
    "--" => Operator::DECREMENT,
    "+=" => Operator::INCREMENTBY,
    "-=" => Operator::DECREMENTBY,
};
