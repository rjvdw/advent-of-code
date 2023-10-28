//! The solution for [advent of code 2020, day 2](https://adventofcode.com/2020/day/2)

use std::path::PathBuf;

use clap::Parser;

use rdcl_aoc_core::input::InputReader;
use rdcl_aoc_core::{err_parse_error, DynResult, MainResult};

use crate::policy::Policy;
use crate::policy_v1::PolicyV1;
use crate::policy_v2::PolicyV2;

mod policy;
mod policy_v1;
mod policy_v2;

#[derive(Parser, Debug)]
#[clap(about = "The solution for advent of code 2020, day 2")]
struct Args {
    /// The file which contains the puzzle input.
    input: PathBuf,
}

fn main() -> MainResult {
    let args: Args = Args::parse();
    let input = InputReader::from(args.input).read_lines();
    let (valid_v1, valid_v2) = check(input)?;

    println!(
        "There are {} valid passwords according to the old policy",
        valid_v1
    );
    println!(
        "There are {} valid passwords according to the new policy",
        valid_v2
    );

    Ok(())
}

fn check<T>(input: T) -> DynResult<(usize, usize)>
where
    T: Iterator<Item = String>,
{
    let mut valid_v1 = 0;
    let mut valid_v2 = 0;

    for line in input {
        let (policy, pw) = match line.find(": ") {
            Some(pos) => {
                let policy = line[..pos].parse::<Policy>()?;
                let pw = &line[pos + 2..];

                (policy, pw)
            }
            None => err_parse_error!("Invalid input: {}", line)?,
        };

        if PolicyV1::check(&policy, pw) {
            valid_v1 += 1;
        }

        if PolicyV2::check(&policy, pw) {
            valid_v2 += 1;
        }
    }

    Ok((valid_v1, valid_v2))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> impl Iterator<Item = String> {
        InputReader::from("./src/day02/test.txt").read_lines()
    }

    #[test]
    fn test_check() {
        assert_eq!(check(test_data()).unwrap(), (2, 1));
    }
}
