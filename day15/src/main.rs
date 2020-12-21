extern crate helpers;

use std::env;
use std::process::exit;

use helpers::handle_result;

const SEEN_SIZE: usize = 100_000_000;

/// https://adventofcode.com/2020/day/15
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <index> ...<inputs>", &args[0]);
        exit(1);
    }

    let mut args = args
        .iter()
        .skip(1)
        .map(|x| handle_result(x.parse::<usize>()));

    let index = args.next().unwrap();
    let inputs = args.collect::<Vec<usize>>();

    println!(
        "The number at position {} is: {}",
        index,
        solve(index, &inputs),
    );
}

fn solve(index: usize, inputs: &[usize]) -> usize {
    let mut seen_at: Vec<usize> = vec![0; SEEN_SIZE];
    let mut last_number = *inputs.last().unwrap();

    for (idx, &number) in inputs.iter().enumerate().rev().skip(1) {
        seen_at[number] = idx + 1;
    }

    for idx in inputs.len()..index {
        let next_number = if seen_at[last_number] == 0 {
            0
        } else {
            idx - seen_at[last_number]
        };

        seen_at[last_number] = idx;
        last_number = next_number;
    }

    last_number
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn test_1() {
            assert_eq!(solve(10, &[0, 3, 6]), 0);
        }

        #[test]
        fn test_2() {
            assert_eq!(solve(2020, &[0, 3, 6]), 436);
        }

        #[test]
        fn test_3() {
            assert_eq!(solve(2020, &[1, 3, 2]), 1);
        }

        #[test]
        fn test_4() {
            assert_eq!(solve(2020, &[2, 1, 3]), 10);
        }

        #[test]
        fn test_5() {
            assert_eq!(solve(2020, &[1, 2, 3]), 27);
        }

        #[test]
        fn test_6() {
            assert_eq!(solve(2020, &[2, 3, 1]), 78);
        }

        #[test]
        fn test_7() {
            assert_eq!(solve(2020, &[3, 2, 1]), 438);
        }

        #[test]
        fn test_8() {
            assert_eq!(solve(2020, &[3, 1, 2]), 1836);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn test_1() {
            assert_eq!(solve(30_000_000, &[0, 3, 6]), 175594);
        }

        #[test]
        fn test_2() {
            assert_eq!(solve(30_000_000, &[1, 3, 2]), 2578);
        }

        #[test]
        fn test_3() {
            assert_eq!(solve(30_000_000, &[2, 1, 3]), 3544142);
        }

        #[test]
        fn test_4() {
            assert_eq!(solve(30_000_000, &[1, 2, 3]), 261214);
        }

        #[test]
        fn test_5() {
            assert_eq!(solve(30_000_000, &[2, 3, 1]), 6895259);
        }

        #[test]
        fn test_6() {
            assert_eq!(solve(30_000_000, &[3, 2, 1]), 18);
        }

        #[test]
        fn test_7() {
            assert_eq!(solve(30_000_000, &[3, 1, 2]), 362);
        }
    }
}
