//! The solution for [advent of code 2022, day 5](https://adventofcode.com/2022/day/5)

use std::path::PathBuf;

use clap::Parser;

use rdcl_aoc_core::error::ParseError;
use rdcl_aoc_core::input::InputReader;
use rdcl_aoc_core::{err_parse_error, MainResult};

use crate::crane::{Crane, CrateMover9000, CrateMover9001};

mod crane;

#[derive(Parser, Debug)]
#[clap(about = "The solution for advent of code 2022, day 5")]
struct Args {
    /// The file which contains the puzzle input.
    input: PathBuf,
}

type Containers = Vec<Vec<char>>;

fn main() -> MainResult {
    let args: Args = Args::parse();
    let input = InputReader::from(args.input);

    let mut lines = input.read_lines();
    let mut containers = parse_starting_configuration(&mut lines)?;

    process(&mut containers, &mut lines, &mut CrateMover9000::default())?;

    println!(
        "Using the CrateMover 9000, the top container in each stack is {}",
        encode(&containers)
    );

    let mut lines = input.read_lines();
    let mut containers = parse_starting_configuration(&mut lines)?;

    process(&mut containers, &mut lines, &mut CrateMover9001::default())?;

    println!(
        "Using the CrateMover 9001, the top container in each stack is {}",
        encode(&containers)
    );

    Ok(())
}

fn encode(containers: &Containers) -> String {
    containers
        .iter()
        .filter(|stack| !stack.is_empty())
        .map(|stack| stack.last().unwrap())
        .collect()
}

fn process<C, T>(
    containers: &mut Containers,
    input: &mut T,
    crane: &mut C,
) -> Result<(), ParseError>
where
    C: Crane,
    T: Iterator<Item = String>,
{
    for line in input {
        let line = check_prefix(&line, "move ")?;
        let (line, mut count) = check_nr(line)?;
        let line = check_prefix(line, "from ")?;
        let (line, from) = check_nr(line)?;
        let from = from - 1;
        let line = check_prefix(line, "to ")?;
        let to = line.parse::<usize>()? - 1;

        while count > 0 {
            match containers[from].pop() {
                Some(v) => crane.push(v),
                None => err_parse_error!("Container {} was empty", from)?,
            }
            count -= 1;
        }

        while let Some(v) = crane.pop() {
            containers[to].push(v);
        }
    }

    Ok(())
}

fn check_prefix<'a>(line: &'a str, prefix: &str) -> Result<&'a str, ParseError> {
    match line.strip_prefix(prefix) {
        Some(v) => Ok(v),
        None => err_parse_error!("Invalid input: {}", line),
    }
}

fn check_nr(line: &str) -> Result<(&str, usize), ParseError> {
    match line.find(' ') {
        Some(p) => {
            let i = line[..p].parse::<usize>()?;
            let line = &line[p + 1..];
            Ok((line, i))
        }
        None => err_parse_error!("Invalid input: {}", line),
    }
}

fn parse_starting_configuration<T>(input: &mut T) -> Result<Containers, ParseError>
where
    T: Iterator<Item = String>,
{
    let mut starting_configuration = vec![];
    let spec: Vec<String> = input.take_while(|l| !l.is_empty()).collect();

    let mut i = 0;
    let mut done = false;
    while !done {
        let p = 4 * i;
        done = true;
        for line in spec.iter().rev().skip(1) {
            if line.len() > p {
                if done {
                    // nothing was done yet this iteration
                    done = false;
                    starting_configuration.push(vec![]);
                }

                match line.chars().nth(p + 1) {
                    Some(ch) if ch != ' ' => {
                        starting_configuration[i].push(ch);
                    }
                    Some(_) => {}
                    _ => {
                        return err_parse_error!("Invalid input: {}", line);
                    }
                }
            }
        }
        i += 1;
    }

    Ok(starting_configuration)
}

#[cfg(test)]
mod tests {
    use rdcl_aoc_core::input::InputReader;

    use super::*;

    fn test_data() -> impl Iterator<Item = String> {
        InputReader::from("./src/day05/test.txt").read_lines()
    }

    #[test]
    fn test_parse_starting_configuration() {
        let starting_configuration = parse_starting_configuration(&mut test_data()).unwrap();

        assert_eq!(
            starting_configuration,
            vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]
        );
    }

    #[test]
    fn test_process_with_crate_mover_9000() {
        let mut input = test_data();
        let mut containers = parse_starting_configuration(&mut input).unwrap();
        process(&mut containers, &mut input, &mut CrateMover9000::default()).unwrap();

        assert_eq!(
            containers,
            vec![vec!['C'], vec!['M'], vec!['P', 'D', 'N', 'Z']],
        );
    }

    #[test]
    fn test_process_with_crate_mover_9001() {
        let mut input = test_data();
        let mut containers = parse_starting_configuration(&mut input).unwrap();
        process(&mut containers, &mut input, &mut CrateMover9001::default()).unwrap();

        assert_eq!(
            containers,
            vec![vec!['M'], vec!['C'], vec!['P', 'Z', 'N', 'D']],
        );
    }
}
