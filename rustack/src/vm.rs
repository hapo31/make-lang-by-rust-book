use crate::parse::Value;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Vm {
    pub stack: Vec<Value>,
    pub vars: HashMap<String, Option<Value>>,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            stack: vec![],
            vars: HashMap::new(),
        }
    }
}
