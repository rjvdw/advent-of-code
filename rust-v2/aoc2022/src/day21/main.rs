//! The solution for [advent of code 2022, day 21](https://adventofcode.com/2022/day/21)

use std::collections::HashMap;
use std::path::PathBuf;

use clap::Parser;

use rdcl_aoc_core::error::ParseError;
use rdcl_aoc_core::input::InputReader;
use rdcl_aoc_core::MainResult;

use crate::monkey::{Monkey, Operation};

mod monkey;

#[derive(Parser, Debug)]
#[clap(about = "The solution for advent of code 2022, day 21")]
struct Args {
    /// The file which contains the puzzle input.
    input: PathBuf,

    /// The name of the root monkey.
    #[clap(long, value_parser, default_value = "root")]
    root: String,

    /// Your name.
    #[clap(long, value_parser, default_value = "humn")]
    you: String,
}

fn main() -> MainResult {
    let args: Args = Args::parse();
    let input = InputReader::from(args.input);

    let mut monkeys = parse(input.read_lines())?;

    let number = run(&monkeys, &args.root).unwrap();
    println!("The monkey called 'root' shouts out the value {}", number);

    fix_input(&mut monkeys, &args.root, &args.you);
    let number = run(&monkeys, &args.root).unwrap();
    println!("The number you should call out is {}", number);

    Ok(())
}

enum Answer {
    Value(i64),
    Expression(Box<dyn FnOnce(i64) -> i64>),
}

impl Answer {
    fn unwrap(&self) -> i64 {
        match self {
            Answer::Value(v) => *v,
            Answer::Expression(_) => panic!("called `unwrap` on an expression"),
        }
    }
}

fn run(monkeys: &HashMap<String, Monkey>, root: &str) -> Answer {
    match monkeys.get(root).unwrap() {
        Monkey::Value(v) => Answer::Value(*v),
        Monkey::Math(op, lhs, rhs) => match (run(monkeys, lhs), run(monkeys, rhs)) {
            (Answer::Value(lhs), Answer::Value(rhs)) => match *op {
                Operation::Add => Answer::Value(lhs + rhs),
                Operation::Sub => Answer::Value(lhs - rhs),
                Operation::Mul => Answer::Value(lhs * rhs),
                Operation::Div => Answer::Value(lhs / rhs),
                Operation::Eq => Answer::Value(i64::from(lhs == rhs)),
            },
            (Answer::Expression(_), Answer::Expression(_)) => {
                panic!("Cannot handle input with multiple variables");
            }
            (Answer::Value(lhs), Answer::Expression(rhs)) => match *op {
                // lhs + humn == root  =>  humn == root - lhs
                Operation::Add => Answer::Expression(Box::new(move |i| rhs(i - lhs))),

                // lhs - humn == root  =>  humn == lhs - root
                Operation::Sub => Answer::Expression(Box::new(move |i| rhs(lhs - i))),

                // lhs * humn == root  =>  humn == root / lhs
                Operation::Mul => Answer::Expression(Box::new(move |i| rhs(i / lhs))),

                // lhs / humn == root  =>  humn == lhs / root
                Operation::Div => Answer::Expression(Box::new(move |i| rhs(lhs / i))),

                // humn == root
                Operation::Eq => Answer::Value(rhs(lhs)),
            },
            (Answer::Expression(lhs), Answer::Value(rhs)) => match *op {
                // humn + rhs == root  =>  humn == root - rhs
                Operation::Add => Answer::Expression(Box::new(move |i| lhs(i - rhs))),

                // humn - rhs == root  =>  humn == root + rhs
                Operation::Sub => Answer::Expression(Box::new(move |i| lhs(i + rhs))),

                // humn * rhs == root  =>  humn == root / lhs
                Operation::Mul => Answer::Expression(Box::new(move |i| lhs(i / rhs))),

                // humn / rhs == root  =>  humn == root * rhs
                Operation::Div => Answer::Expression(Box::new(move |i| lhs(i * rhs))),

                // humn == root
                Operation::Eq => Answer::Value(lhs(rhs)),
            },
        },
        Monkey::Var => Answer::Expression(Box::new(|i| i)),
    }
}

fn parse<T>(input: T) -> Result<HashMap<String, Monkey>, ParseError>
where
    T: Iterator<Item = String>,
{
    let mut monkeys: HashMap<String, Monkey> = HashMap::new();

    for monkey in input {
        let label = monkey[..4].to_string();
        let value = monkey[6..].parse::<Monkey>()?;
        monkeys.insert(label, value);
    }

    Ok(monkeys)
}

fn fix_input(monkeys: &mut HashMap<String, Monkey>, root: &str, you: &str) {
    if let Some(Monkey::Math(op, _, _)) = monkeys.get_mut(root) {
        *op = Operation::Eq;
    }

    if let Some(m) = monkeys.get_mut(you) {
        *m = Monkey::Var;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> HashMap<String, Monkey> {
        parse(InputReader::from("./src/day21/test.txt").read_lines()).unwrap()
    }

    #[test]
    fn test_run_without_you() {
        assert_eq!(run(&test_data(), "root").unwrap(), 152);
    }

    #[test]
    fn test_run_with_you() {
        let mut monkeys = test_data();
        fix_input(&mut monkeys, "root", "humn");
        assert_eq!(run(&monkeys, "root").unwrap(), 301);
    }
}
