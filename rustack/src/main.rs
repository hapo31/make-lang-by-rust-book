use parse::ParseContext;

mod eval;
mod parse;
mod vm;

fn main() {
    for line in std::io::stdin().lines().flatten() {
        let mut parse_context = ParseContext::new();
        let (codes, _) = parse::parse(
            &line.split_whitespace().collect::<Vec<_>>(),
            &mut parse_context,
        );
        println!("{codes:?}");
        let mut vm: vm::Vm<'_> = vm::Vm::new(&mut parse_context);
        for code in codes.to_block() {
            eval::eval(code, &mut vm);
        }

        println!("{vm:?}");
    }
}
