use crate::op::*;
use crate::parse::Value;
use crate::vm::Vm;

pub fn eval<'src>(code: Value<'src>, vm: &mut Vm<'src>) {
    match code {
        Value::Num(num) => vm.stack.push(Value::Num(num)),
        Value::Op(op) => match op {
            "+" => add(vm),
            "-" => sub(vm),
            "*" => mul(vm),
            "/" => div(vm),
            "if" => op_if(vm),
            _ => panic!("Unknown operator: {}", op),
        },
        Value::Sym(sym) => {}
        Value::Block(block) => {
            for value in block {
                eval(value, vm);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parse::Value;

    #[test]
    fn test_eval() {
        let mut vm = Vm::new();
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
        let mut vm = Vm::new();
        eval(Value::Num(1), &mut vm);
        eval(Value::Block(vec![Value::Num(1)]), &mut vm);
        eval(Value::Block(vec![Value::Num(2)]), &mut vm);
        eval(Value::Op("if"), &mut vm);
        assert_eq!(vm.stack, vec![Value::Num(1)]);
    }
}
