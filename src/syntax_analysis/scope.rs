pub struct JvVariable {
    pub var_type: String,
    pub var_name: String,
}

pub struct JvFunction {
    pub return_type: String,
    pub func_name: String,
}

//A scope contains a reference to it's parent (also a scope),
//a list of variables and a list of functions
pub struct Scope {
    pub parent: Box<Option<Scope>>,
    pub variables: Vec<JvVariable>,
    pub functions: Vec<JvFunction>,
}

impl Scope {
    pub fn new(parent: Option<Scope>) -> Self {
        Scope {
            parent: Box::new(parent),
            variables: Vec::new(),
            functions: Vec::new(),
        }
    }
}
