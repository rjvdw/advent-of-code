//! The solution for [advent of code 2022, day 9](https://adventofcode.com/2022/day/9)

use std::cmp::Ordering;
use std::collections::HashSet;
use std::path::PathBuf;
use std::process::exit;

use clap::Parser;

use rdcl_aoc_core::error::ParseError;
use rdcl_aoc_core::input::InputReader;
use rdcl_aoc_core::{err_parse_error, MainResult};

#[derive(Parser, Debug)]
#[clap(about = "The solution for advent of code 2022, day 9")]
struct Args {
    /// The file which contains the puzzle input.
    input: PathBuf,

    /// The number of knots in the rope.
    #[clap(short, long, value_parser, default_value_t = 2)]
    knots: usize,
}

fn main() -> MainResult {
    let args: Args = Args::parse();
    let input = InputReader::from(args.input);

    if args.knots == 0 {
        eprintln!("There must be at least one knot!");
        exit(1);
    }

    let visited = travel(input.read_lines(), args.knots)?;

    println!(
        "With {} knots, the tail will visit {} places",
        args.knots, visited
    );

    Ok(())
}

fn travel<T>(input: T, knots: usize) -> Result<usize, ParseError>
where
    T: Iterator<Item = String>,
{
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut rope = vec![(0, 0); knots];
    visited.insert((0, 0));

    for line in input {
        let n = line[2..].parse::<usize>()?;
        let d = match line.chars().next() {
            Some('U') => (1, 0),
            Some('R') => (0, 1),
            Some('D') => (-1, 0),
            Some('L') => (0, -1),
            _ => {
                return err_parse_error!("Invalid input: {}", line);
            }
        };

        for _ in 0..n {
            let mut it = rope.iter_mut();

            let mut head = it.next().unwrap();
            head.0 += d.0;
            head.1 += d.1;

            for tail in it {
                if !are_adjacent(*head, *tail) {
                    tail.0 = move_towards(tail.0, head.0);
                    tail.1 = move_towards(tail.1, head.1);
                }
                head = tail;
            }

            visited.insert(*head);
        }
    }

    Ok(visited.len())
}

fn are_adjacent(a: (i32, i32), b: (i32, i32)) -> bool {
    a.0.abs_diff(b.0) <= 1 && a.1.abs_diff(b.1) <= 1
}

fn move_towards(tail: i32, head: i32) -> i32 {
    match tail.cmp(&head) {
        Ordering::Less => tail + 1,
        Ordering::Equal => tail,
        Ordering::Greater => tail - 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data_1() -> impl Iterator<Item = String> {
        InputReader::from("./src/day09/test-1.txt").read_lines()
    }

    fn test_data_2() -> impl Iterator<Item = String> {
        InputReader::from("./src/day09/test-2.txt").read_lines()
    }

    #[test]
    fn test_travel_with_2_knots() {
        assert_eq!(travel(test_data_1(), 2).unwrap(), 13);
    }

    #[test]
    fn test_travel_with_10_knots_1() {
        assert_eq!(travel(test_data_1(), 10).unwrap(), 1);
    }

    #[test]
    fn test_travel_with_10_knots_2() {
        assert_eq!(travel(test_data_2(), 10).unwrap(), 36);
    }
}
