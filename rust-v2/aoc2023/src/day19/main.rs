//! The solution for [advent of code 2023, day 19](https://adventofcode.com/2023/day/19)

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use clap::Parser;

use rdcl_aoc_core::error::ParseError;
use rdcl_aoc_core::input::InputReader;
use rdcl_aoc_core::MainResult;

use crate::parts::Part;
use crate::workflow::{Label, Workflow};

mod parts;
mod str_encoder;
mod workflow;

const START_LABEL: &str = "in";

#[derive(Parser, Debug)]
#[clap(about = "The solution for advent of code 2023, day 19")]
struct Args {
    /// The file which contains the puzzle input.
    input: PathBuf,
}

fn main() -> MainResult {
    let args: Args = Args::parse();
    let input = InputReader::from(args.input).read_lines();

    let (workflows, part_list) = parse_input(input)?;
    println!(
        "The sum of the scores is {}",
        workflows.eval_part_list(&part_list, START_LABEL)
    );

    Ok(())
}

type Workflows = HashMap<Label, Workflow>;

trait WorkflowMap {
    fn eval_part_list(&self, part_list: &Vec<Part>, start: &str) -> u32 {
        let mut scores = 0;
        for part in part_list {
            if self.accepts_part(part, start) {
                scores += part.score();
            }
        }
        scores
    }

    fn accepts_part(&self, part: &Part, start: &str) -> bool;
}

impl WorkflowMap for Workflows {
    fn accepts_part(&self, part: &Part, start: &str) -> bool {
        let mut seen: HashSet<Label> = HashSet::new();
        let mut current = start
            .parse::<Label>()
            .expect("Invalid starting label provided");
        seen.insert(current);

        while let Some(workflow) = self.get(&current) {
            match workflow.eval(part) {
                Label::Accepted => {
                    return true;
                }
                Label::Rejected => {
                    return false;
                }
                label => {
                    if seen.contains(&label) {
                        panic!("an infinite loop was encountered");
                    }
                    seen.insert(label);
                    current = label;
                }
            }
        }

        unreachable!("an invalid label was encountered: {}", current)
    }
}

fn parse_input<T>(input: T) -> Result<(Workflows, Vec<Part>), ParseError>
where
    T: Iterator<Item = String>,
{
    let mut parsing_workflows = true;
    let mut workflows = Workflows::new();
    let mut part_list = vec![];

    for line in input {
        if line.is_empty() {
            parsing_workflows = false;
        } else if parsing_workflows {
            let workflow = line.parse::<Workflow>()?;
            workflows.insert(workflow.label(), workflow);
        } else {
            part_list.push(line.parse()?);
        }
    }

    Ok((workflows, part_list))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> (Workflows, Vec<Part>) {
        parse_input(InputReader::from("./src/day19/test.txt").read_lines()).unwrap()
    }

    #[test]
    fn test_accepts_part() {
        let (workflows, part_list) = test_data();

        assert!(workflows.accepts_part(&part_list[0], START_LABEL));
        assert!(!workflows.accepts_part(&part_list[1], START_LABEL));
        assert!(workflows.accepts_part(&part_list[2], START_LABEL));
        assert!(!workflows.accepts_part(&part_list[3], START_LABEL));
        assert!(workflows.accepts_part(&part_list[4], START_LABEL));
    }

    #[test]
    fn test_eval_part_list() {
        let (workflows, part_list) = test_data();

        assert_eq!(workflows.eval_part_list(&part_list, START_LABEL), 19114);
    }
}
