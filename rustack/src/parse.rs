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
}

pub fn parse<'src, 'a>(input: &'src [&'a str]) -> (Value, &'src [&'a str]) {
    let mut tokens = vec![];
    let mut words = &input;
    let mut rest: &'src [&'a str];

    while let Some((&word, rest_slice)) = words.split_first() {
        rest = rest_slice;
        if word.is_empty() {
            break;
        }

        match word {
            "{" => {
                let value;
                (value, rest) = parse(rest);
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
                } else {
                    sym_parse(word)
                };

                tokens.push(code);
            }
        }
        words = &rest;
    }
    (Value::Block(tokens), &[])
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
            (
                Value::Block(vec![num!(1), num!(2), op!("+"), block![num!(3), num!(4)]]),
                &[] as &[&str]
            )
        );
    }

    #[test]
    fn test_vardef_parse() {
        let input = &["/a", "1", "def"];
        assert_eq!(
            parse(input),
            (block![sym!("a"), num!(1), op!("def")], &[] as &[&str])
        );
    }
}
