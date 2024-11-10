use crate::op::*;
use crate::parse::Value;

pub fn eval<'src>(code: Value<'src>, stack: &mut Vec<Value<'src>>) {
    match code {
        Value::Num(num) => stack.push(Value::Num(num)),
        Value::Op(op) => match op {
            "+" => add(stack),
            "-" => sub(stack),
            "*" => mul(stack),
            "/" => div(stack),
            _ => panic!("Unknown operator: {}", op),
        },
        Value::Block(block) => {
            for value in block {
                eval(value, stack);
            }
        }
    }
}
