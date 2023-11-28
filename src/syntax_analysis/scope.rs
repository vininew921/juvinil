#[derive(Clone)]
pub struct JvVariable {
    pub var_type: String, //Type of the variable
    pub var_name: String, //Name of the variable
    pub assigned: bool,   //Flag to check if the variable value has been assigned
}

#[derive(Clone)]
pub struct JvFunction {
    pub return_type: String, //Return type of the function
    pub func_name: String,   //Name of the function
    pub params: Vec<String>, //Parameters of the function (just their type, not name)
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
