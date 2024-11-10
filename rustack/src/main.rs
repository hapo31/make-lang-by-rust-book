use parse::parse;
mod op;
mod parse;

fn main() {
    for line in std::io::stdin().lines().flatten() {
        let (stack, _) = parse(&line.split_whitespace().collect::<Vec<_>>());

        println!("{stack:?}");
    }
}
