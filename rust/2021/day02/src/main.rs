extern crate rdcl_aoc_helpers;

use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::input::WithReadLines;

use crate::direction::Direction;
use crate::instruction::Instruction;

mod direction;
mod instruction;

/// https://adventofcode.com/2021/day/2
fn main() {
    let args = get_args(&["<input file>"], 1);

    let instructions = File::open(&args[1])
        .read_lines(1)
        .collect::<Vec<Instruction>>();

    let (depth, position) = compute_depth_and_position(&instructions);
    println!(
        "Not considering aim, the submarine ends at position {} and depth {}, for a final answer of {}.",
        position,
        depth,
        position * depth
    );

    let (depth, position) = compute_depth_and_position_with_aim(&instructions);
    println!(
        "Considering aim, the submarine ends at position {} and depth {}, for a final answer of {}.",
        position,
        depth,
        position * depth
    );
}

fn compute_depth_and_position(instructions: &[Instruction]) -> (i32, i32) {
    let mut depth = 0;
    let mut position = 0;

    for instruction in instructions {
        match instruction.direction {
            Direction::Forward => position += instruction.distance,
            Direction::Down => depth += instruction.distance,
            Direction::Up => depth -= instruction.distance,
        }
    }

    (depth, position)
}

fn compute_depth_and_position_with_aim(instructions: &[Instruction]) -> (i32, i32) {
    let mut aim = 0;
    let mut depth = 0;
    let mut position = 0;

    for instruction in instructions {
        match instruction.direction {
            Direction::Forward => {
                position += instruction.distance;
                depth += instruction.distance * aim;
            }
            Direction::Down => aim += instruction.distance,
            Direction::Up => aim -= instruction.distance,
        }
    }

    (depth, position)
}

#[cfg(test)]
mod tests {
    use rdcl_aoc_helpers::input::WithAsRecords;

    use super::*;

    #[test]
    fn test_compute_depth_and_position() {
        let test_input = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ]
        .as_records::<Instruction>()
        .unwrap();

        assert_eq!(compute_depth_and_position(&test_input), (10, 15));
    }

    #[test]
    fn test_compute_depth_and_position_with_aim() {
        let test_input = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ]
        .as_records::<Instruction>()
        .unwrap();

        assert_eq!(compute_depth_and_position_with_aim(&test_input), (60, 15));
    }
}
