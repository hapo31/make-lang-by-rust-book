mod stack;

fn main() {
    let mut stack = vec![];

    for line in std::io::stdin().lines() {
        if let Ok(line) = line {
            let words: Vec<_> = line.split(' ').collect();

            for word in words {
                if let Ok(parsed) = word.parse::<i32>() {
                    stack.push(parsed);
                } else {
                    match word {
                        "+" => stack::add(&mut stack),
                        "-" => stack::sub(&mut stack),
                        "*" => stack::mul(&mut stack),
                        "/" => stack::div(&mut stack),
                        _ => panic!("{word:?} could not be parsed."),
                    }
                }
            }

            println!("Stack: {stack:?}");
        }
    }
}
