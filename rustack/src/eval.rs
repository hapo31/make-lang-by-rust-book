use crate::num;
use crate::parse::Value;
use crate::vm::Vm;

pub fn eval_with_codes(codes: Vec<Value>, vm: &mut Vm) {
    for code in codes {
        eval_impl(&code, vm);
    }
}

pub fn eval_with_block(code: Value, vm: &mut Vm) {
    eval_with_codes(code.to_block(), vm);
}

fn eval_impl(code: &Value, vm: &mut Vm) {
    match code {
        Value::Op(op) => match op.as_str() {
            "+" => add(vm),
            "-" => sub(vm),
            "*" => mul(vm),
            "/" => div(vm),
            "<" => lt(vm),
            ">" => gt(vm),
            "if" => op_if(vm),
            "def" => op_def(vm),
            "puts" => op_puts(vm),
            _ => panic!("Unknown operator: {}", op),
        },
        Value::Sym(sym) => {
            // 変数がきちんとコード中で定義されているか
            match vm.vars.get(sym.as_str()) {
                // 変数が定義されている場合は変数の値をスタックに積む
                Some(Some(value)) => vm.stack.push(value.clone()), // ここの clone() は num!(value.as_num())) と同じことである
                // 変数がまだ定義されていない場合はそのままスタックに積む
                _ => vm.stack.push(code.clone()),
            }
        }
        Value::Block(block) => {
            for value in block {
                eval_impl(value, vm);
            }
        }
        _ => vm.stack.push(code.clone()),
    }
}

macro_rules! impl_op {
    {$name:ident, $op:tt} => {
        fn $name(vm: &mut Vm) {
            let lhs = vm.stack.pop().unwrap().as_num();
            let rhs = vm.stack.pop().unwrap().as_num();

            vm.stack.push(num!((lhs $op rhs) as i32));
        }
    };
}

impl_op! (add, +);
impl_op! (sub, -);
impl_op! (mul, *);
impl_op! (div, /);
impl_op! (lt, <);
impl_op! (gt, >);

fn op_def(vm: &mut Vm) {
    let value = vm.stack.pop().unwrap();
    let sym = vm.stack.pop().unwrap();
    vm.vars.insert(sym.to_sym(), Some(value));
}

fn op_if(vm: &mut Vm) {
    let false_branch = vm.stack.pop().unwrap().to_block();
    let true_branch = vm.stack.pop().unwrap().to_block();
    let cond = vm.stack.pop().unwrap().to_block();

    for code in &cond {
        eval_impl(code, vm);
    }

    let cond_result = vm.stack.pop().unwrap().as_num();

    if cond_result != 0 {
        for code in &true_branch {
            eval_impl(code, vm);
        }
    } else {
        for code in &false_branch {
            eval_impl(code, vm);
        }
    }
}

fn op_puts(vm: &mut Vm) {
    let value = vm.stack.pop().unwrap();
    println!("{}", value.to_string());
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{block, op, parse::Value, sym};

    fn new_test<'src, 'a>(code: Vec<Value>) -> Vm {
        let mut vm = Vm::new();
        eval_with_codes(code, &mut vm);
        vm
    }

    #[test]
    fn test_add() {
        let mut vm = new_test(vec![num!(1), num!(2)]);

        add(&mut vm);
        assert_eq!(vm.stack, vec![num!(3)]);
    }

    #[test]
    fn test_sub() {
        let mut vm = new_test(vec![num!(1), num!(2)]);
        sub(&mut vm);
        assert_eq!(vm.stack, vec![num!(1)]);
    }

    #[test]
    fn test_mul() {
        let mut vm = new_test(vec![num!(2), num!(3)]);
        mul(&mut vm);
        assert_eq!(vm.stack, vec![num!(6)]);
    }

    #[test]
    fn test_div() {
        let mut vm = new_test(vec![num!(3), num!(6)]);
        div(&mut vm);
        assert_eq!(vm.stack, vec![num!(2)]);
    }

    #[test]
    fn test_op_if_true() {
        let mut vm = new_test(vec![block![num!(1)], block![num!(2)], block![num!(1)]]);
        op_if(&mut vm);
        assert_eq!(vm.stack, vec![num!(2)]);
    }

    #[test]
    fn test_op_if_false() {
        let mut vm = new_test(vec![block![num!(0)], block![num!(2)], block![num!(1)]]);
        op_if(&mut vm);
        assert_eq!(vm.stack, vec![num!(1)]);
    }

    #[test]
    fn test_eval() {
        let vm = new_test(vec![
            num!(4),
            num!(4),
            op!("-"),
            num!(4),
            num!(2),
            op!("+"),
            num!(3),
            op!("/"),
            num!(4),
            op!("*"),
            op!("*"),
        ]);
        assert_eq!(vm.stack, vec![num!(0)]);
    }

    #[test]
    fn test_eval_if() {
        let vm = new_test(vec![num!(1), block![num!(1)], block![num!(2)], op!("if")]);

        assert_eq!(vm.stack, vec![num!(1)]);
    }

    #[test]
    fn test_eval_def() {
        let vm = new_test(vec![
            sym!("a"),
            num!(1),
            op!("def"),
            sym!("b"),
            num!(2),
            op!("def"),
            sym!("a"),
            sym!("b"),
            op!("+"),
        ]);

        assert_eq!(vm.stack, vec![num!(3)]);
        assert_eq!(vm.vars.get("a"), Some(&Some(num!(1))));
        assert_eq!(vm.vars.get("b"), Some(&Some(num!(2))));
    }
}
