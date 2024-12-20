use std::io::{BufRead, BufReader};

use eval::eval_with_block as eval;
use parse::parse;
use vm::Vm;

mod eval;
mod parse;
mod vm;

fn main() {
    if let Some(f) = std::env::args()
        .nth(1)
        .and_then(|f| std::fs::File::open(f).ok())
    {
        run_batch(BufReader::new(f));
    } else {
        repl();
    }
}

fn run_batch(buffer: impl BufRead) {
    let mut vm: Vm = Vm::new();
    for line in buffer.lines().flatten() {
        let parse_result = parse(line.split_whitespace().collect::<Vec<_>>().as_slice());

        if let Ok(codes) = parse_result {
            eval(codes, &mut vm);
        } else if let Err(parse_error) = parse_result {
            eprintln!("{}", parse_error.to_string());
        }
    }
}

fn repl() {
    let mut vm: Vm = Vm::new();
    for line in std::io::stdin().lines().flatten() {
        let parse_result = parse(line.split_whitespace().collect::<Vec<_>>().as_slice());

        if let Ok(codes) = parse_result {
            eval(codes, &mut vm);
        } else if let Err(parse_error) = parse_result {
            eprintln!("{}", parse_error.to_string());
        }
    }
}
