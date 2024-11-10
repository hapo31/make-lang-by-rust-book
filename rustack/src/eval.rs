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
