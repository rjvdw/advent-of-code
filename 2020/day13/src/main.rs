extern crate helpers;

use std::env;
use std::process::exit;

use helpers::handle_result;

use crate::math::solve_crt;

mod input_parser;
mod math;

/// https://adventofcode.com/2020/day/13
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <input file>", &args[0]);
        exit(1);
    }

    let (earliest_departure, schedule) = handle_result(input_parser::read(&args[1]));

    match find_shortest_wait_time(earliest_departure, &schedule) {
        Some((id, wait_time)) => println!(
            "Solution for part 1: {} x {} = {}",
            id,
            wait_time,
            id * wait_time,
        ),
        None => println!("Could not find a solution for part 1"),
    }

    println!("The answer to the contest is: {}", win_contest(&schedule));
}

fn find_shortest_wait_time(earliest_departure: u32, schedule: &[u32]) -> Option<(u32, u32)> {
    schedule
        .iter()
        .filter(|&&v| v != 0)
        .map(|&v| (v, v - (earliest_departure % v)))
        .min_by_key(|&v| v.1)
}

/// Find the solution to the contest. We assume all bus IDs are pairwise coprime.
fn win_contest(schedule: &[u32]) -> u64 {
    let input = schedule
        .iter()
        .enumerate()
        .filter(|&(_, &a)| a != 0)
        .map(|(a, &n)| (n as u64, a as u64)) // swap the inputs and cast to i64
        .collect::<Vec<(u64, u64)>>();

    match input.first() {
        Some(&first) => {
            let (p, n) = input
                .iter()
                .skip(1)
                .fold(first, |p1, &p2| (p1.0 * p2.0, solve_crt(p1, p2)));

            p - n
        }
        None => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn test() {
            let earliest_departure = 939;
            let schedule = vec![7, 13, 0, 0, 59, 0, 31, 19];

            assert_eq!(
                find_shortest_wait_time(earliest_departure, &schedule),
                Some((59, 5))
            );
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn test_1() {
            let schedule = vec![7, 13, 0, 0, 59, 0, 31, 19];

            assert_eq!(win_contest(&schedule), 1068781);
        }

        #[test]
        fn test_2() {
            let schedule = vec![17, 0, 13, 19];

            assert_eq!(win_contest(&schedule), 3417);
        }

        #[test]
        fn test_3() {
            let schedule = vec![67, 7, 59, 61];

            assert_eq!(win_contest(&schedule), 754018);
        }

        #[test]
        fn test_4() {
            let schedule = vec![67, 0, 7, 59, 61];

            assert_eq!(win_contest(&schedule), 779210);
        }

        #[test]
        fn test_5() {
            let schedule = vec![67, 7, 0, 59, 61];

            assert_eq!(win_contest(&schedule), 1261476);
        }

        #[test]
        fn test_6() {
            let schedule = vec![1789, 37, 47, 1889];

            assert_eq!(win_contest(&schedule), 1202161486);
        }
    }
}
