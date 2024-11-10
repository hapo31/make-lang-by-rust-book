use crate::{eval::eval, parse::Value};

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

pub fn op_if(stack: &mut Vec<Value>) {
    let false_branch = stack.pop().unwrap().to_block();
    let true_branch = stack.pop().unwrap().to_block();
    let cond = stack.pop().unwrap().to_block();

    for code in cond {
        eval(code, stack);
    }

    let cond_result = stack.pop().unwrap().as_num();

    if cond_result != 0 {
        for code in true_branch {
            eval(code, stack);
        }
    } else {
        for code in false_branch {
            eval(code, stack);
        }
    }
}

mod test {
    use super::*;
    #[test]
    fn test_add() {
        let mut stack = vec![Value::Num(1), Value::Num(2)];
        add(&mut stack);
        assert_eq!(stack, vec![Value::Num(3)]);
    }

    #[test]
    fn test_sub() {
        let mut stack = vec![Value::Num(1), Value::Num(2)];
        sub(&mut stack);
        assert_eq!(stack, vec![Value::Num(1)]);
    }

    #[test]
    fn test_mul() {
        let mut stack = vec![Value::Num(2), Value::Num(3)];
        mul(&mut stack);
        assert_eq!(stack, vec![Value::Num(6)]);
    }

    #[test]
    fn test_div() {
        let mut stack = vec![Value::Num(3), Value::Num(6)];
        div(&mut stack);
        assert_eq!(stack, vec![Value::Num(2)]);
    }

    #[test]
    fn test_op_if_true() {
        {
            let mut stack = vec![
                Value::Block(vec![Value::Num(1)]),
                Value::Block(vec![Value::Num(2)]),
                Value::Block(vec![Value::Num(1)]),
            ];
            op_if(&mut stack);
            assert_eq!(stack, vec![Value::Num(2)]);
        }
    }

    #[test]
    fn test_op_if_false() {
        {
            let mut stack = vec![
                Value::Block(vec![Value::Num(0)]),
                Value::Block(vec![Value::Num(2)]),
                Value::Block(vec![Value::Num(1)]),
            ];
            op_if(&mut stack);
            assert_eq!(stack, vec![Value::Num(1)]);
        }
    }
}
