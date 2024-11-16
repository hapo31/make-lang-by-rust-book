use crate::parse::Value;
use crate::vm::Vm;

pub fn eval<'src>(code: Value<'src>, vm: &mut Vm<'src>) {
    match code {
        Value::Op(op) => match op {
            "+" => add(vm),
            "-" => sub(vm),
            "*" => mul(vm),
            "/" => div(vm),
            "<" => lt(vm),
            "if" => op_if(vm),
            "def" => op_def(vm),
            _ => panic!("Unknown operator: {}", op),
        },
        Value::Sym(sym) => {
            // 変数がきちんとコード中で定義されているか
            if let Some(declared) = vm.declares.get(sym) {
                if *declared {
                    // その文脈で変数が定義済みであれば、変数の値をスタックに積む
                    let value = vm.vars.get(sym).unwrap();
                    vm.stack.push(Value::Num(value.as_num()));
                }
            } else {
                panic!("{sym:?} is not declared.");
            }
            // いずれの場合にも当てはまらなければそのままスタックに積む
            vm.stack.push(code)
        }
        Value::Block(block) => {
            for value in block {
                eval(value, vm);
            }
        }
        _ => vm.stack.push(code),
    }
}

macro_rules! impl_op {
    {$name:ident, $op:tt} => {
        fn $name(vm: &mut Vm) {
            let lhs = vm.stack.pop().unwrap().as_num();
            let rhs = vm.stack.pop().unwrap().as_num();

            vm.stack.push(Value::Num((lhs $op rhs) as i32));
        }
    };
}

impl_op! (add, +);
impl_op! (sub, -);
impl_op! (mul, *);
impl_op! (div, /);
impl_op! (lt, <);

fn op_def(vm: &mut Vm) {
    let value = vm.stack.pop().unwrap();
    let sym = vm.stack.pop().unwrap().to_sym();

    vm.vars.insert(sym, value);
    vm.declares.entry(sym).and_modify(|e| *e = true);
}

fn op_if(vm: &mut Vm) {
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::parse::{self, Value};

    #[test]
    fn test_add() {
        let mut parse_context = parse::ParseContext::new();
        let mut vm = Vm::from_code(vec![Value::Num(1), Value::Num(2)], &mut parse_context);
        add(&mut vm);
        assert_eq!(vm.stack, vec![Value::Num(3)]);
    }

    #[test]
    fn test_sub() {
        let mut parse_context = parse::ParseContext::new();
        let mut vm = Vm::from_code(vec![Value::Num(1), Value::Num(2)], &mut parse_context);
        sub(&mut vm);
        assert_eq!(vm.stack, vec![Value::Num(1)]);
    }

    #[test]
    fn test_mul() {
        let mut parse_context = parse::ParseContext::new();
        let mut vm = Vm::from_code(vec![Value::Num(2), Value::Num(3)], &mut parse_context);
        mul(&mut vm);
        assert_eq!(vm.stack, vec![Value::Num(6)]);
    }

    #[test]
    fn test_div() {
        let mut parse_context = parse::ParseContext::new();
        let mut vm = Vm::from_code(vec![Value::Num(3), Value::Num(6)], &mut parse_context);
        div(&mut vm);
        assert_eq!(vm.stack, vec![Value::Num(2)]);
    }

    #[test]
    fn test_op_if_true() {
        let mut parse_context = parse::ParseContext::new();
        let mut vm = Vm::from_code(
            vec![
                Value::Block(vec![Value::Num(1)]),
                Value::Block(vec![Value::Num(2)]),
                Value::Block(vec![Value::Num(1)]),
            ],
            &mut parse_context,
        );
        op_if(&mut vm);
        assert_eq!(vm.stack, vec![Value::Num(2)]);
    }

    #[test]
    fn test_op_if_false() {
        let mut parse_context = parse::ParseContext::new();
        let mut vm = Vm::from_code(
            vec![
                Value::Block(vec![Value::Num(0)]),
                Value::Block(vec![Value::Num(2)]),
                Value::Block(vec![Value::Num(1)]),
            ],
            &mut parse_context,
        );
        op_if(&mut vm);
        assert_eq!(vm.stack, vec![Value::Num(1)]);
    }

    #[test]
    fn test_eval() {
        let mut parse_context = parse::ParseContext::new();
        let mut vm = Vm::new(&mut parse_context);
        eval(Value::Num(4), &mut vm);
        eval(Value::Num(4), &mut vm);
        eval(Value::Op("-"), &mut vm);
        eval(Value::Num(4), &mut vm);
        eval(Value::Num(2), &mut vm);
        eval(Value::Op("+"), &mut vm);
        eval(Value::Num(3), &mut vm);
        eval(Value::Op("/"), &mut vm);
        eval(Value::Num(4), &mut vm);
        eval(Value::Op("*"), &mut vm);
        eval(Value::Op("*"), &mut vm);
        assert_eq!(vm.stack, vec![Value::Num(0)]);
    }

    #[test]
    fn test_eval_if() {
        let mut parse_context = parse::ParseContext::new();
        let mut vm = Vm::new(&mut parse_context);
        eval(Value::Num(1), &mut vm);
        eval(Value::Block(vec![Value::Num(1)]), &mut vm);
        eval(Value::Block(vec![Value::Num(2)]), &mut vm);
        eval(Value::Op("if"), &mut vm);
        assert_eq!(vm.stack, vec![Value::Num(1)]);
    }
}
