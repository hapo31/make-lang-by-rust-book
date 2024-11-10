use std::collections::HashMap;

use crate::parse::Value;

#[derive(Debug)]
pub struct Vm<'src> {
    pub stack: Vec<Value<'src>>,
    pub vars: HashMap<&'src str, Value<'src>>,
}

impl<'src> Vm<'src> {
    pub fn new() -> Self {
        Self {
            stack: vec![],
            vars: HashMap::new(),
        }
    }

    pub fn from_code(code: Vec<Value<'src>>) -> Self {
        Self {
            stack: code,
            vars: HashMap::new(),
        }
    }
}
