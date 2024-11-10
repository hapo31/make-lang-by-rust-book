pub fn add(stack: &mut Vec<i32>) {
    let lhs = stack.pop().unwrap();
    let rhs = stack.pop().unwrap();

    stack.push(lhs + rhs);
}

pub fn sub(stack: &mut Vec<i32>) {
    let lhs = stack.pop().unwrap();
    let rhs = stack.pop().unwrap();

    stack.push(lhs - rhs);
}

pub fn mul(stack: &mut Vec<i32>) {
    let lhs = stack.pop().unwrap();
    let rhs = stack.pop().unwrap();

    stack.push(lhs * rhs);
}

pub fn div(stack: &mut Vec<i32>) {
    let lhs = stack.pop().unwrap();
    let rhs = stack.pop().unwrap();

    stack.push(lhs / rhs);
}
