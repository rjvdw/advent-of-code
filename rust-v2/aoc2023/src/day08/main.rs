//! The solution for [advent of code 2023, day 8](https://adventofcode.com/2023/day/8)

use std::collections::HashMap;
use std::path::PathBuf;

use clap::Parser;

use rdcl_aoc_core::error::ParseError;
use rdcl_aoc_core::input::InputReader;
use rdcl_aoc_core::{assert_or_parse_error, err_parse_error, MainResult};
use rdcl_aoc_math::lcm;

use crate::map::{Direction, Label, Node};

mod map;

const START: &str = "AAA";
const END: &str = "ZZZ";

const GHOST_START: char = 'A';
const GHOST_END: char = 'Z';

#[derive(Parser, Debug)]
#[clap(about = "The solution for advent of code 2023, day 8")]
struct Args {
    /// The file which contains the puzzle input.
    input: PathBuf,
}

fn main() -> MainResult {
    let args: Args = Args::parse();
    let mut input = InputReader::from(args.input).read_lines();

    let instructions = parse_instructions(&mut input)?;
    let map = parse_map(input)?;

    let route = follow_instructions(&instructions, &map, START, END);
    match route {
        Some(steps) => println!(
            "It takes {} steps to travel from {} to {}",
            steps, START, END
        ),
        None => println!("There is no path from {} to {}", START, END),
    }

    let route = follow_ghostly_instructions(&instructions, &map, GHOST_START, GHOST_END);
    match route {
        Some(steps) => println!(
            "It takes {} steps to travel from xx{} to xx{}",
            steps, GHOST_START, GHOST_END
        ),
        None => println!("There is no path from xx{} to xx{}", GHOST_START, GHOST_END),
    }

    Ok(())
}

fn follow_instructions(
    instructions: &[Direction],
    map: &HashMap<Label, Node>,
    from: &str,
    to: &str,
) -> Option<usize> {
    let mut steps = 0;
    let mut current = from.to_string();
    let mut index = 0;

    while current != to {
        steps += 1;
        current = map.get(&current)?.travel(instructions[index]);
        index = (index + 1) % instructions.len();
    }

    Some(steps)
}

fn follow_ghostly_instructions(
    instructions: &[Direction],
    map: &HashMap<Label, Node>,
    from: char,
    to: char,
) -> Option<usize> {
    let starting_positions = map
        .keys()
        .filter(|key| key.ends_with(from))
        .cloned()
        .collect::<Vec<_>>();

    let mut steps = 0;
    let mut current = starting_positions.clone();
    let mut index = 0;

    // in order to reduce run time, keep track of loops that have been encountered
    let mut detected_loops: Vec<Option<usize>> = vec![None; starting_positions.len()];

    while !is_done(&current, &detected_loops, &mut steps, to) {
        let mut next = Vec::with_capacity(current.len());
        steps += 1;

        for (i, label) in current.iter().enumerate() {
            let next_node = map.get(label)?.travel(instructions[index]);
            next.push(next_node.clone());

            // if the next node is the same as the start node, then a loop has been detected
            let l = detected_loops.get_mut(i)?;
            if l.is_none() && next_node.ends_with(to) {
                *l = Some(steps);
            }
        }

        index = (index + 1) % instructions.len();
        current = next;
    }

    Some(steps)
}

/// Check if all necessary calculations are done.
///
/// If a loop has been detected for every single path, then the entire
/// process can be short-circuited.
fn is_done(
    labels: &[Label],
    detected_loops: &[Option<usize>],
    steps: &mut usize,
    to: char,
) -> bool {
    let all_loops_detected = detected_loops.iter().all(|o| o.is_some());
    if all_loops_detected {
        *steps = detected_loops
            .iter()
            .map(|o| o.unwrap())
            .reduce(lcm)
            .unwrap();

        true
    } else {
        labels.iter().all(|label| label.ends_with(to))
    }
}

fn parse_instructions<T>(input: &mut T) -> Result<Vec<Direction>, ParseError>
where
    T: Iterator<Item = String>,
{
    let mut result = vec![];
    for (i, ch) in input.next().ok_or(())?.chars().enumerate() {
        match ch {
            'L' => result.push(Direction::Left),
            'R' => result.push(Direction::Right),
            _ => return err_parse_error!("invalid instruction @ {}: {}", i, ch),
        }
    }

    // remove the empty line that separates the instructions from the rest of the map
    assert_or_parse_error!(input.next() == Some("".to_string()));

    Ok(result)
}

fn parse_map<T>(input: T) -> Result<HashMap<Label, Node>, ParseError>
where
    T: Iterator<Item = String>,
{
    let mut result = HashMap::new();
    for line in input {
        let from = line[0..3].to_string();
        let left = line[7..10].to_string();
        let right = line[12..15].to_string();

        result.insert(from, Node::new(left, right));
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data_1() -> impl Iterator<Item = String> {
        InputReader::from("./src/day08/test1.txt").read_lines()
    }

    fn test_data_2() -> impl Iterator<Item = String> {
        InputReader::from("./src/day08/test2.txt").read_lines()
    }

    fn test_data_3() -> impl Iterator<Item = String> {
        InputReader::from("./src/day08/test3.txt").read_lines()
    }

    #[test]
    fn test_follow_1() {
        let mut input = test_data_1();
        let instructions = parse_instructions(&mut input).unwrap();
        let map = parse_map(input).unwrap();

        assert_eq!(
            follow_instructions(&instructions, &map, START, END),
            Some(2usize)
        );
    }

    #[test]
    fn test_follow_2() {
        let mut input = test_data_2();
        let instructions = parse_instructions(&mut input).unwrap();
        let map = parse_map(input).unwrap();

        assert_eq!(
            follow_instructions(&instructions, &map, START, END),
            Some(6usize)
        );
    }

    #[test]
    fn test_follow_3() {
        let mut input = test_data_3();
        let instructions = parse_instructions(&mut input).unwrap();
        let map = parse_map(input).unwrap();

        assert_eq!(
            follow_ghostly_instructions(&instructions, &map, 'A', 'Z'),
            Some(6usize)
        );
    }
}
