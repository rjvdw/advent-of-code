extern crate helpers;

use std::env;
use std::process::exit;

use helpers::{handle_result, read_input};

/// https://adventofcode.com/2020/day/1
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: {} <input file> <count> <sum>", &args[0]);
        exit(1);
    }

    let path = &args[1];
    let count = handle_result(args[2].parse::<usize>());
    let sum = handle_result(args[3].parse::<i32>());
    let values: Vec<i32> = handle_result(read_input(path));

    match solve(&values, 0, count, sum) {
        Some(v) => println!("{}", v),
        None => println!("No solution"),
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(solve(&input, 0, 2, 2020), Some(514579));
    }

    #[test]
    fn test_part_2() {
        let input = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(solve(&input, 0, 3, 2020), Some(241861950));
    }
}
