pub struct JvVariable {
    pub var_type: String,
    pub var_name: String,
}

pub struct Scope {
    pub parent: Box<Option<Scope>>,
    pub variables: Vec<JvVariable>,
}

impl Scope {
    pub fn new(parent: Option<Scope>) -> Self {
        Scope {
            parent: Box::new(parent),
            variables: Vec::new(),
        }
    }
}
