mod eval;
mod op;
mod parse;
mod vm;

fn main() {
    for line in std::io::stdin().lines().flatten() {
        let (codes, _) = parse::parse(&line.split_whitespace().collect::<Vec<_>>());
        // println!("{codes:?}");
        let mut vm: vm::Vm<'_> = vm::Vm::new();
        for code in codes.to_block() {
            eval::eval(code, &mut vm);
        }

        println!("{vm:?}");
    }
}
