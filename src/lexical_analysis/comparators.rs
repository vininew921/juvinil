use phf::phf_map;

#[derive(Debug, Clone, Copy)]
pub enum Comparator {
    AND,
    OR,
    EQ,
    NEQ,
    BGTHAN,
    SMTHAN,
    BGEQTHAN,
    SMEQTHAN,
}

pub static COMPARATORS: phf::Map<&'static str, Comparator> = phf_map! {
    "&&" => Comparator::AND,
    "||" => Comparator::OR,
    "==" => Comparator::EQ,
    "!=" => Comparator::NEQ,
    "<" => Comparator::SMTHAN,
    ">" => Comparator::BGTHAN,
    ">=" => Comparator::BGEQTHAN,
    "<=" => Comparator::SMEQTHAN,
};
