mod eval;
mod op;
mod parse;

fn main() {
    for line in std::io::stdin().lines().flatten() {
        let (codes, _) = parse::parse(&line.split_whitespace().collect::<Vec<_>>());
        // println!("{codes:?}");
        let stack = &mut vec![];
        for code in codes.to_block() {
            eval::eval(code, stack);
        }

        println!("{stack:?}");
    }
}
