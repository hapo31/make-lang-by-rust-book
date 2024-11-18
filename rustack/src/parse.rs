use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Num(i32),
    Op(String),
    Sym(String),
    Block(Vec<Value>),
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
pub struct ParseContext<'src> {
    pub vars: HashSet<&'src str>,
}

impl<'src> ParseContext<'src> {
    pub fn new() -> Self {
        Self {
            vars: HashSet::new(),
        }
    }

    pub fn add_var(&mut self, var: &'src str) -> Result<(), ()> {
        match self.vars.insert(var) {
            true => Ok(()),
            false => Err(()),
        }
    }

    // 今のところテストでしか使ってないが、実コードでも使うようになったら cfg 外す
    #[cfg(test)]
    pub fn has_var(&self, var: &str) -> bool {
        self.vars.contains(var)
    }
}

impl<'src> Value {
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

    pub fn to_sym(&self) -> &String {
        match self {
            Self::Sym(sym) => sym,
            _ => panic!("Value is not a symbol, actual: {self:?}"),
        }
    }
}

pub fn parse<'src, 'a>(
    input: &'a [&'src str],
    context: &mut ParseContext<'src>,
) -> (Value, &'a [&'src str]) {
    let mut tokens = vec![];
    let mut words = input;

    while let Some((&word, mut rest)) = words.split_first() {
        if word.is_empty() {
            break;
        }

        match word {
            "{" => {
                let value;
                (value, rest) = parse(rest, context);
                tokens.push(value);
            }
            "}" => {
                return (Value::Block(tokens), rest);
            }
            _ => {
                let code = if let Ok(value) = num_parse(word) {
                    value
                } else if let Ok(op) = op_parse(word) {
                    op
                } else if let Ok(sym) = sym_parse(word, context) {
                    sym
                } else {
                    panic!("{:?} could not be parsed.", word);
                };

                tokens.push(code);
            }
        }
        words = rest;
    }
    (Value::Block(tokens), words)
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
        _ => Err(word),
    }
}

fn num_parse(word: &str) -> Result<Value, ()> {
    match word.parse::<i32>() {
        Ok(num) => Ok(Value::Num(num)),
        Err(_) => Err(()),
    }
}

fn sym_parse<'src>(word: &'src str, context: &mut ParseContext<'src>) -> Result<Value, ()> {
    if let Some(word) = word.strip_prefix("/") {
        match context.add_var(word) {
            Ok(()) => Ok(sym!(&word[0..])),
            Err(()) => panic!("{:?} is already declared.", word),
        }
    } else {
        Ok(sym!(&word))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_num_parse() {
        assert_eq!(num_parse("123"), Ok(num!(123)));
        assert_eq!(num_parse("abc"), Err(()));
    }

    #[test]
    fn test_block_parse() {
        let mut parse_context = ParseContext::new();
        let input = &["1", "2", "+", "{", "3", "4", "}"];
        assert_eq!(
            parse(input, &mut parse_context),
            (
                Value::Block(vec![num!(1), num!(2), op!("+"), block![num!(3), num!(4)]]),
                &[] as &[&str]
            )
        );
    }

    #[test]
    fn test_vardef_parse() {
        let mut parse_context = ParseContext::new();
        let input = &["/a", "1", "def"];
        assert_eq!(
            parse(input, &mut parse_context),
            (block![sym!("a"), num!(1), op!("def")], &[] as &[&str])
        );
        assert!(parse_context.has_var("a"));
    }
}
