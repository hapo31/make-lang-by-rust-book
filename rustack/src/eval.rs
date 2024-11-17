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
            ">" => gt(vm),
            "if" => op_if(vm),
            "def" => op_def(vm),
            _ => panic!("Unknown operator: {}", op),
        },
        Value::Sym(sym) => {
            // 変数がきちんとコード中で定義されているか
            match vm.declares.get(sym) {
                // 変数が定義されている場合はその値をスタックに積む
                Some(true) => vm
                    .stack
                    .push(Value::Num(vm.vars.get(sym).unwrap().as_num())),
                // 変数がまだ定義されていない場合はそのままスタックに積む
                Some(false) => vm.stack.push(code),
                // 変数がどこにも定義されていない場合は実行時エラー
                None => panic!("{sym:?} is not declared."),
            }
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
impl_op! (gt, >);

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

    fn new_test<'src, 'a>(code: Vec<Value<'src>>, declared_vars: &'a [&'src str]) -> Vm<'src> {
        let mut parse_context = parse::ParseContext::new();

        for vars in declared_vars {
            match parse_context.add_var(*vars) {
                Ok(()) => {}
                Err(()) => panic!("{:?} is already declared.", vars),
            }
        }

        let vm = Vm::from_code(code, &parse_context);
        vm
    }

    fn eval_codes<'a>(codes: &Vec<Value<'a>>, vm: &mut Vm<'a>) {
        for code in codes {
            eval(code.clone(), vm);
        }
    }

    #[test]
    fn test_add() {
        let mut vm = new_test(vec![Value::Num(1), Value::Num(2)], &[]);

        add(&mut vm);
        assert_eq!(vm.stack, vec![Value::Num(3)]);
    }

    #[test]
    fn test_sub() {
        let mut vm = new_test(vec![Value::Num(1), Value::Num(2)], &[]);
        sub(&mut vm);
        assert_eq!(vm.stack, vec![Value::Num(1)]);
    }

    #[test]
    fn test_mul() {
        let mut vm = new_test(vec![Value::Num(2), Value::Num(3)], &[]);
        mul(&mut vm);
        assert_eq!(vm.stack, vec![Value::Num(6)]);
    }

    #[test]
    fn test_div() {
        let mut vm = new_test(vec![Value::Num(3), Value::Num(6)], &[]);
        div(&mut vm);
        assert_eq!(vm.stack, vec![Value::Num(2)]);
    }

    #[test]
    fn test_op_if_true() {
        let mut vm = new_test(
            vec![
                Value::Block(vec![Value::Num(1)]),
                Value::Block(vec![Value::Num(2)]),
                Value::Block(vec![Value::Num(1)]),
            ],
            &[],
        );
        op_if(&mut vm);
        assert_eq!(vm.stack, vec![Value::Num(2)]);
    }

    #[test]
    fn test_op_if_false() {
        let mut vm = new_test(
            vec![
                Value::Block(vec![Value::Num(0)]),
                Value::Block(vec![Value::Num(2)]),
                Value::Block(vec![Value::Num(1)]),
            ],
            &[],
        );
        op_if(&mut vm);
        assert_eq!(vm.stack, vec![Value::Num(1)]);
    }

    #[test]
    fn test_eval() {
        let mut vm = new_test(vec![], &[]);
        eval_codes(
            &vec![
                Value::Num(4),
                Value::Num(4),
                Value::Op("-"),
                Value::Num(4),
                Value::Num(2),
                Value::Op("+"),
                Value::Num(3),
                Value::Op("/"),
                Value::Num(4),
                Value::Op("*"),
                Value::Op("*"),
            ],
            &mut vm,
        );
        assert_eq!(vm.stack, vec![Value::Num(0)]);
    }

    #[test]
    fn test_eval_if() {
        let mut vm = new_test(vec![], &[]);
        eval_codes(
            &vec![
                Value::Num(1),
                Value::Block(vec![Value::Num(1)]),
                Value::Block(vec![Value::Num(2)]),
                Value::Op("if"),
            ],
            &mut vm,
        );
    }

    #[test]
    fn test_eval_def() {
        let mut vm = new_test(vec![], &["a", "b"]);

        eval_codes(
            &vec![
                Value::Sym("a"),
                Value::Num(1),
                Value::Op("def"),
                Value::Sym("b"),
                Value::Num(2),
                Value::Op("def"),
                Value::Sym("a"),
                Value::Sym("b"),
                Value::Op("+"),
            ],
            &mut vm,
        );
        assert_eq!(vm.stack, vec![Value::Num(3)]);
        assert_eq!(vm.vars.get("a"), Some(&Value::Num(1)));
        assert_eq!(vm.vars.get("b"), Some(&Value::Num(2)));
        assert_eq!(vm.declares.get("a"), Some(&true));
        assert_eq!(vm.declares.get("b"), Some(&true));
    }
}
