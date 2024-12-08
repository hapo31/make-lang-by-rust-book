#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Num(i32),
    Op(String),
    Sym(String),
    Block(Vec<Value>),
}

impl Value {
    pub fn as_num(&self) -> i32 {
        match self {
            Self::Num(num) => *num,
            _ => panic!("Value is not a number, actual: {self:?}"),
        }
    }
    pub fn to_block(self) -> Vec<Value> {
        match self {
            Self::Block(block) => block,
            _ => vec![self],
        }
    }

    pub fn to_sym(self) -> String {
        match self {
            Self::Sym(sym) => sym,
            _ => panic!("Value is not a symbol, actual: {self:?}"),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Num(num) => num.to_string(),
            Self::Op(ref s) | Self::Sym(ref s) => s.clone(),
            Self::Block(_) => String::from("<block>"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    UnmatchedBranckets(String),
}

impl ParseError {
    pub fn to_string(&self) -> String {
        match self {
            Self::UnmatchedBranckets(msg) => msg.clone(),
        }
    }
}

#[macro_export]
macro_rules! num {
    ($value:expr) => {
        Value::Num($value)
    };
}

#[macro_export]
macro_rules! op {
    ($value:expr) => {
        Value::Op($value.to_string())
    };
}

#[macro_export]
macro_rules! sym {
    ($value:expr) => {
        Value::Sym($value.to_string())
    };
}

#[macro_export]
macro_rules! block {
    ($($value:expr),*) => {
        Value::Block(vec![$($value),*])
    };
}

#[derive(Debug)]
pub struct ParseContext {
    pub blocks: Vec<Vec<Value>>,
}

impl ParseContext {
    pub fn new() -> Self {
        Self { blocks: vec![] }
    }

    pub fn push_block(&mut self) -> &Vec<Value> {
        self.blocks.push(vec![]);
        self.blocks.last().unwrap()
    }

    pub fn pop_block(&mut self) -> Option<Vec<Value>> {
        if self.blocks.len() == 0 {
            return None;
        }
        self.blocks.pop()
    }

    pub fn push(&mut self, value: Value) {
        self.blocks.last_mut().unwrap().push(value);
    }
}

pub fn parse<'src, 'a>(input: &'src [&'a str]) -> Result<Value, ParseError> {
    let mut context = ParseContext::new();
    let mut words = &input;
    let mut rest;

    context.push_block();

    while let Some((&word, rest_slice)) = words.split_first() {
        rest = rest_slice;
        if word.is_empty() {
            break;
        }

        match word {
            "{" => {
                context.push_block();
            }
            "}" => {
                if let Some(block) = context.pop_block() {
                    context.push(Value::Block(block));
                } else {
                    return Err(ParseError::UnmatchedBranckets("Unmatched '}'".to_string()));
                }
            }
            _ => {
                let code = if let Ok(value) = num_parse(word) {
                    value
                } else if let Ok(op) = op_parse(word) {
                    op
                } else {
                    sym_parse(word)
                };

                context.push(code);
            }
        }
        words = &rest;
    }
    Ok(Value::Block(context.pop_block().unwrap()))
}

fn op_parse(word: &str) -> Result<Value, &str> {
    match word {
        "+" => Ok(op!("+")),
        "-" => Ok(op!("-")),
        "*" => Ok(op!("*")),
        "/" => Ok(op!("/")),
        "<" => Ok(op!("<")),
        ">" => Ok(op!(">")),
        "if" => Ok(op!("if")),
        "def" => Ok(op!("def")),
        "puts" => Ok(op!("puts")),
        _ => Err(word),
    }
}

fn num_parse(word: &str) -> Result<Value, ()> {
    match word.parse::<i32>() {
        Ok(num) => Ok(Value::Num(num)),
        Err(_) => Err(()),
    }
}

fn sym_parse<'src>(word: &'src str) -> Value {
    if let Some(word) = word.strip_prefix("/") {
        sym!(&word[0..])
    } else {
        sym!(&word)
    }
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::*;

    #[test]
    fn test_num_parse() {
        assert_eq!(num_parse("123"), Ok(num!(123)));
        assert_eq!(num_parse("abc"), Err(()));
    }

    #[test]
    fn test_block_parse() {
        let input = &["1", "2", "+", "{", "3", "4", "}"];
        assert_eq!(
            parse(input),
            Ok(Value::Block(vec![
                num!(1),
                num!(2),
                op!("+"),
                block![num!(3), num!(4)]
            ]))
        );
    }

    #[test]
    fn test_vardef_parse() {
        let input = &["/a", "1", "def"];
        assert_eq!(parse(input), Ok(block![sym!("a"), num!(1), op!("def")]));
    }
}
