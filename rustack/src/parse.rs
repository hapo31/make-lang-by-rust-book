#[derive(Debug, PartialEq, Eq)]
pub enum Value<'src> {
    Num(i32),
    Op(&'src str),
    Sym(&'src str),
    Block(Vec<Value<'src>>),
}

impl<'src> Value<'src> {
    pub fn as_num(&self) -> i32 {
        match self {
            Self::Num(num) => *num,
            _ => panic!("Value is not a number"),
        }
    }
    pub fn to_block(self) -> Vec<Value<'src>> {
        match self {
            Self::Block(block) => block,
            _ => panic!("Value is not a block"),
        }
    }
}

fn op_parse<'src>(word: &'src str) -> Result<Value<'src>, &'src str> {
    match word {
        "+" => Ok(Value::Op("+")),
        "-" => Ok(Value::Op("-")),
        "*" => Ok(Value::Op("*")),
        "/" => Ok(Value::Op("/")),
        _ => Err(word),
    }
}

fn num_parse<'src>(word: &'src str) -> Result<Value<'src>, ()> {
    match word.parse::<i32>() {
        Ok(num) => Ok(Value::Num(num)),
        Err(_) => Err(()),
    }
}

pub fn parse<'src, 'a>(input: &'a [&'src str]) -> (Value<'src>, &'a [&'src str]) {
    let mut tokens = vec![];
    let mut words = input;

    while let Some((&word, mut rest)) = words.split_first() {
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
                if let Ok(value) = num_parse(word) {
                    tokens.push(value);
                } else if let Ok(op) = op_parse(word) {
                    tokens.push(op);
                } else {
                    panic!("{:?} could not be parsed.", word);
                }
            }
        }
        words = rest;
    }
    (Value::Block(tokens), words)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_num_parse() {
        assert_eq!(num_parse("123"), Ok(Value::Num(123)));
        assert_eq!(num_parse("abc"), Err(()));
    }

    #[test]
    fn test_block_parse() {
        let input = &["1", "2", "+", "{", "3", "4", "}"];
        assert_eq!(
            parse(input),
            (
                Value::Block(vec![
                    Value::Num(1),
                    Value::Num(2),
                    Value::Op("+"),
                    Value::Block(vec![Value::Num(3), Value::Num(4)])
                ]),
                &input[0..0]
            )
        );
    }
}
