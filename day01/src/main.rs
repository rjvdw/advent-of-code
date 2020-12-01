use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: {} <input file> <count> <sum>", &args[0]);
        exit(1);
    }

    let path = &args[1];
    let count = *&args[2].parse::<usize>().expect("Invalid count");
    let sum = *&args[3].parse::<i32>().expect("Invalid sum");
    let values = read_input(path);

    match solve(&values, 0, count, sum) {
        Some(v) => println!("{}", v),
        None => println!("No solution")
    }
}

fn read_input(path: &String) -> Vec<i32> {
    let file = File::open(path).expect("Cannot read input");
    let mut values = Vec::new();
    for line in BufReader::new(file).lines() {
        let line = line.expect("Unable to read line");
        let line = line.parse::<i32>().expect("Unable to parse value");
        values.push(line);
    }

    return values;
}

/// Solves the problem: Does there exist a subset of `count` numbers in `values`, such that they sum
/// to `sum`. If so, return the product of these numbers.
fn solve(values: &Vec<i32>, skip: usize, count: usize, sum: i32) -> Option<i32> {
    assert_ne!(count, 0);
    assert!(sum > 0);

    if count == 1 {
        for &v in values.iter().skip(skip).filter(|&&v| v == sum) {
            return Some(v);
        }
    } else {
        for (pos, &v) in values.iter().skip(skip).filter(|&&v| v < sum).enumerate() {
            match solve(values, pos + 1, count - 1, sum - v) {
                Some(r) => return Some(r * v),
                None => {}
            }
        }
    }

    return None;
}
