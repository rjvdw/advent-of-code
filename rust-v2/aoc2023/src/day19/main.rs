//! The solution for [advent of code 2023, day 19](https://adventofcode.com/2023/day/19)

use std::path::PathBuf;

use clap::Parser;

use rdcl_aoc_core::input::InputReader;
use rdcl_aoc_core::{MainResult, ParseResult};

use crate::parts::{parse_part, HasRatings, Part};
use crate::workflows::{Acceptable, Workflow, Workflows};

mod parts;
mod workflows;

#[derive(Parser, Debug)]
#[clap(about = "The solution for advent of code 2023, day 19")]
struct Args {
    /// The file which contains the puzzle input.
    input: PathBuf,

    /// The lower bound to use for ratings.
    #[clap(long, short, value_parser, default_value_t = 1)]
    lower_bound: usize,

    /// The upper bound to use for ratings.
    #[clap(long, short, value_parser, default_value_t = 4000)]
    upper_bound: usize,
}

fn main() -> MainResult {
    let args: Args = Args::parse();
    let (workflows, parts) = parse_input(&mut InputReader::from(args.input).read_lines())?;
    let bounds = (args.lower_bound, args.upper_bound);

    println!(
        "The sum of all ratings of all accepted parts is {}",
        parts
            .iter()
            .filter(|p| workflows.accepts(p))
            .map(|p| p.sum_ratings())
            .sum::<usize>()
    );
    println!(
        "The total number of parts that will be accepted is {}",
        workflows.count_acceptable(bounds)
    );

    Ok(())
}

type ParsedInput = (Workflows, Vec<Part>);
type ParseInputResult = ParseResult<ParsedInput>;

fn parse_input<T>(input: &mut T) -> ParseInputResult
where
    T: Iterator<Item = String>,
{
    let mut workflows = Workflows::new();
    for workflow in input.take_while(|line| !line.is_empty()) {
        let workflow = workflow.parse::<Workflow>()?;
        workflows.insert(workflow.name(), workflow);
    }

    let mut parts = vec![];
    for part in input {
        parts.push(parse_part(&part)?);
    }

    Ok((workflows, parts))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> ParsedInput {
        parse_input(&mut InputReader::from("./src/day19/test.txt").read_lines()).unwrap()
    }

    #[test]
    fn test_workflows_accepts() {
        let (workflows, parts) = test_data();
        let results = parts
            .iter()
            .map(|part| workflows.accepts(part))
            .collect::<Vec<_>>();

        assert_eq!(results, vec![true, false, true, false, true]);
    }

    #[test]
    fn test_workflows_count_acceptable() {
        let (workflows, _) = test_data();

        assert_eq!(workflows.count_acceptable((1, 4000)), 167_409_079_868_000);
    }
}
