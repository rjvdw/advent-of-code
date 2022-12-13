//! The solution for [advent of code 2022, day 13](https://adventofcode.com/2022/day/13)

use std::cmp::Ordering;
use std::path::PathBuf;

use clap::Parser;

use rdcl_aoc_core::error::ParseError;
use rdcl_aoc_core::input::InputReader;
use rdcl_aoc_core::{err_parse_error, MainResult};

use crate::packet::Packet;

mod packet;

#[derive(Parser, Debug)]
#[clap(about = "The solution for advent of code 2022, day 13")]
struct Args {
    /// The file which contains the puzzle input.
    input: PathBuf,

    /// The value in the first divider packet.
    #[clap(long, value_parser, default_value = "[[2]]")]
    divider1: Packet,

    /// The value in the second divider packet.
    #[clap(long, value_parser, default_value = "[[6]]")]
    divider2: Packet,
}

fn main() -> MainResult {
    let args: Args = Args::parse();
    let input = InputReader::from(args.input);
    let (indices, p1, p2) = check_input(input.read_lines(), args.divider1, args.divider2)?;

    println!(
        "The sum of the indices that are in the correct order: {:?}",
        indices.iter().sum::<usize>()
    );

    println!("The decoder key is {}", p1 * p2);

    Ok(())
}

fn check_input<T>(
    mut input: T,
    divider1: Packet,
    divider2: Packet,
) -> Result<(Vec<usize>, usize, usize), ParseError>
where
    T: Iterator<Item = String>,
{
    let mut indices = vec![];
    let mut p1 = 1;
    let mut p2 = 1;
    let mut i = 1;

    match divider1.cmp(&divider2) {
        Ordering::Less => {
            p2 += 1;
        }
        Ordering::Equal => {}
        Ordering::Greater => {
            p1 += 1;
        }
    }

    while let Some(line1) = input.next() {
        if line1.is_empty() {
            continue;
        }

        match input.next() {
            Some(line2) => {
                let packet1 = line1.parse::<Packet>()?;
                let packet2 = line2.parse::<Packet>()?;

                if packet1 < divider1 {
                    p1 += 1;
                }

                if packet2 < divider1 {
                    p1 += 1;
                }

                if packet1 < divider2 {
                    p2 += 1;
                }

                if packet2 < divider2 {
                    p2 += 1;
                }

                if packet1 < packet2 {
                    indices.push(i);
                }

                i += 1;
            }
            None => {
                return err_parse_error!("Missing a second input line at index {}", i);
            }
        }
    }

    Ok((indices, p1, p2))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> impl Iterator<Item = String> {
        InputReader::from("./src/day13/test.txt").read_lines()
    }

    #[test]
    fn test_check_input() {
        let divider1 = "[[2]]".parse().unwrap();
        let divider2 = "[[6]]".parse().unwrap();

        assert_eq!(
            check_input(test_data(), divider1, divider2).unwrap(),
            (vec![1, 2, 4, 6], 10, 14)
        );
    }
}
