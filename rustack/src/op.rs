use crate::parse::Value;

pub fn add(stack: &mut Vec<Value>) {
    let lhs = stack.pop().unwrap().as_num();
    let rhs = stack.pop().unwrap().as_num();

    stack.push(Value::Num(lhs + rhs));
}

pub fn sub(stack: &mut Vec<Value>) {
    let lhs = stack.pop().unwrap().as_num();
    let rhs = stack.pop().unwrap().as_num();

    stack.push(Value::Num(lhs - rhs));
}

pub fn mul(stack: &mut Vec<Value>) {
    let lhs = stack.pop().unwrap().as_num();
    let rhs = stack.pop().unwrap().as_num();

    stack.push(Value::Num(lhs * rhs));
}

pub fn div(stack: &mut Vec<Value>) {
    let lhs = stack.pop().unwrap().as_num();
    let rhs = stack.pop().unwrap().as_num();

    stack.push(Value::Num(lhs / rhs));
}
