//! The solution for [advent of code 2022, day 22](https://adventofcode.com/2022/day/22)

use std::path::PathBuf;

use clap::Parser;

use rdcl_aoc_core::error::ParseError;
use rdcl_aoc_core::input::InputReader;
use rdcl_aoc_core::{err_parse_error, MainResult};

use crate::grove::Grove;

mod direction;
mod face;
mod grove;
mod next;
mod tile;

#[derive(Parser, Debug)]
#[clap(about = "The solution for advent of code 2022, day 22")]
struct Args {
    /// The file which contains the puzzle input.
    ///
    /// This solution assumes the input will always have the following shape:
    /// ```text
    ///    [1][3]
    ///    [2]
    /// [4][6]
    /// [5]
    /// ```
    /// (where each `[#]` represents a 50x50 square).
    ///
    /// This means the following wrappings should be used for part 2:
    ///
    /// - If you leave `[1]` while facing up, go to `[5]` facing right.
    /// - If you leave `[1]` while facing left, go to `[4]` facing right.
    /// - If you leave `[2]` while facing left, go to `[4]` facing down.
    /// - If you leave `[2]` while facing right, go to `[3]` facing up.
    /// - If you leave `[3]` while facing up, go to `[5]` facing up.
    /// - If you leave `[3]` while facing down, go to `[2]` facing left.
    /// - If you leave `[3]` while facing right, go to `[6]` facing left.
    /// - If you leave `[4]` while facing up, go to `[2]` facing right.
    /// - If you leave `[4]` while facing left, go to `[1]` facing right.
    /// - If you leave `[5]` while facing down, go to `[3]` facing down.
    /// - If you leave `[5]` while facing left, go to `[1]` facing down.
    /// - If you leave `[5]` while facing right, go to `[6]` facing up.
    /// - If you leave `[6]` while facing down, go to `[5]` facing left.
    /// - If you leave `[6]` while facing right, go to `[3]` facing left.
    input: PathBuf,
}

fn main() -> MainResult {
    let args: Args = Args::parse();
    let input = InputReader::from(args.input);

    let (mut grove, instructions) = parse(input.read_lines(), false)?;
    grove.follow_instructions(instructions);
    println!("The password is {}", grove.password());

    let (mut grove, instructions) = parse(input.read_lines(), true)?;
    grove.follow_instructions(instructions);
    println!("The password is {}", grove.password());

    Ok(())
}

fn parse<T>(mut input: T, is_cube: bool) -> Result<(Grove, String), ParseError>
where
    T: Iterator<Item = String>,
{
    let grove = Grove::parse(&mut input, is_cube)?;
    let instructions = match input.next() {
        Some(v) => v,
        None => {
            return err_parse_error!("Invalid input, missing line with instructions.");
        }
    };
    if let Some(line) = input.next() {
        err_parse_error!("Unexpected extra line after input: {}", line)
    } else {
        Ok((grove, instructions))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> impl Iterator<Item = String> {
        InputReader::from("./src/day22/test.txt").read_lines()
    }

    /// Contains the same data as `test_data`, but transformed in such a way that it matches the
    /// expected input shape.
    fn test_data_alt() -> impl Iterator<Item = String> {
        InputReader::from("./src/day22/test-alt.txt").read_lines()
    }

    #[test]
    fn test_password() {
        let (mut grove, instructions) = parse(test_data(), false).unwrap();
        grove.follow_instructions(instructions);

        assert_eq!(grove.password(), 6032);
    }

    #[test]
    fn test_password_cube() {
        let (mut grove, instructions) = parse(test_data_alt(), true).unwrap();
        grove.follow_instructions(instructions);

        assert_eq!(grove.password(), 10006);
    }
}
