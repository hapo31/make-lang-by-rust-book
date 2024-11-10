use crate::{eval::eval, parse::Value, vm::Vm};

pub fn add(vm: &mut Vm) {
    let lhs = vm.stack.pop().unwrap().as_num();
    let rhs = vm.stack.pop().unwrap().as_num();

    vm.stack.push(Value::Num(lhs + rhs));
}

pub fn sub(vm: &mut Vm) {
    let lhs = vm.stack.pop().unwrap().as_num();
    let rhs = vm.stack.pop().unwrap().as_num();

    vm.stack.push(Value::Num(lhs - rhs));
}

pub fn mul(vm: &mut Vm) {
    let lhs = vm.stack.pop().unwrap().as_num();
    let rhs = vm.stack.pop().unwrap().as_num();

    vm.stack.push(Value::Num(lhs * rhs));
}

pub fn div(vm: &mut Vm) {
    let lhs = vm.stack.pop().unwrap().as_num();
    let rhs = vm.stack.pop().unwrap().as_num();

    vm.stack.push(Value::Num(lhs / rhs));
}

pub fn op_if(vm: &mut Vm) {
    let false_branch = vm.stack.pop().unwrap().to_block();
    let true_branch = vm.stack.pop().unwrap().to_block();
    let cond = vm.stack.pop().unwrap().to_block();

    for code in cond {
        eval(code, vm);
    }

    let cond_result = vm.stack.pop().unwrap().as_num();

    if cond_result != 0 {
        for code in true_branch {
            eval(code, vm);
        }
    } else {
        for code in false_branch {
            eval(code, vm);
        }
    }
}

mod test {
    use super::*;
    #[test]
    fn test_add() {
        let mut vm = Vm::from_code(vec![Value::Num(1), Value::Num(2)]);
        add(&mut vm);
        assert_eq!(vm.stack, vec![Value::Num(3)]);
    }

    #[test]
    fn test_sub() {
        let mut vm = Vm::from_code(vec![Value::Num(1), Value::Num(2)]);
        sub(&mut vm);
        assert_eq!(vm.stack, vec![Value::Num(1)]);
    }

    #[test]
    fn test_mul() {
        let mut vm = Vm::from_code(vec![Value::Num(2), Value::Num(3)]);
        mul(&mut vm);
        assert_eq!(vm.stack, vec![Value::Num(6)]);
    }

    #[test]
    fn test_div() {
        let mut vm = Vm::from_code(vec![Value::Num(3), Value::Num(6)]);
        div(&mut vm);
        assert_eq!(vm.stack, vec![Value::Num(2)]);
    }

    #[test]
    fn test_op_if_true() {
        {
            let mut vm = Vm::from_code(vec![
                Value::Block(vec![Value::Num(1)]),
                Value::Block(vec![Value::Num(2)]),
                Value::Block(vec![Value::Num(1)]),
            ]);
            op_if(&mut vm);
            assert_eq!(vm.stack, vec![Value::Num(2)]);
        }
    }

    #[test]
    fn test_op_if_false() {
        {
            let mut vm = Vm::from_code(vec![
                Value::Block(vec![Value::Num(0)]),
                Value::Block(vec![Value::Num(2)]),
                Value::Block(vec![Value::Num(1)]),
            ]);
            op_if(&mut vm);
            assert_eq!(vm.stack, vec![Value::Num(1)]);
        }
    }
}
