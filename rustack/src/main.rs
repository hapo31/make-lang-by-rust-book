use eval::eval_with_block as eval;
use parse::parse;
use vm::Vm;

mod eval;
mod parse;
mod vm;

fn main() {
    let mut vm: Vm = Vm::new();
    for line in std::io::stdin().lines().flatten() {
        let (codes, _) = parse(line.split_whitespace().collect::<Vec<_>>().as_slice());
        println!("{codes:?}");

        eval(codes, &mut vm);

        println!("{vm:?}");
    }
}
