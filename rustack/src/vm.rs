use std::collections::HashMap;

use crate::parse::ParseContext;
use crate::parse::Value;

#[derive(Debug)]
pub struct Vm<'src> {
    pub stack: Vec<Value>,
    pub declares: HashMap<&'src str, bool>,
    pub vars: HashMap<&'src str, Value>,
}

impl<'src> Vm<'src> {
    pub fn new(parse_context: &ParseContext<'src>) -> Self {
        Self {
            stack: vec![],
            vars: HashMap::new(),
            declares: Self::make_declares(parse_context),
        }
    }

    #[cfg(test)]
    pub fn from_code(code: Vec<Value>, parse_context: &ParseContext<'src>) -> Self {
        Self {
            stack: code,
            vars: HashMap::new(),
            declares: Self::make_declares(parse_context),
        }
    }

    fn make_declares(parse_context: &ParseContext<'src>) -> HashMap<&'src str, bool> {
        let mut declares = HashMap::<&'src str, _>::new();
        parse_context.vars.iter().for_each(|var| {
            declares.insert(*var, false);
        });

        declares
    }
}
