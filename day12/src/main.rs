extern crate helpers;

use std::env;
use std::process::exit;

use helpers::{handle_result, read_input};

use crate::coordinates::Coordinates;
use crate::input_record::InputRecord;

mod coordinates;
mod input_record;

/// https://adventofcode.com/2020/day/12
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <input file> <waypoint x>,<waypoint y>", &args[0]);
        exit(1);
    }

    let instructions: Vec<InputRecord> = handle_result(read_input(&args[1]));
    let waypoint = handle_result(args[2].parse::<Coordinates>());

    println!(
        "Manhattan distance without using waypoint: {}",
        travel(&instructions).manhattan_distance(),
    );

    println!(
        "Manhattan distance when using waypoint: {}",
        travel_with_waypoint(&instructions, waypoint).manhattan_distance(),
    );
}

fn travel(instructions: &[InputRecord]) -> Coordinates {
    instructions
        .iter()
        .fold(
            (coordinates::ORIGIN, 0),
            |(position, heading), instruction| instruction.travel(position, heading),
        )
        .0
}

fn travel_with_waypoint(instructions: &[InputRecord], waypoint: Coordinates) -> Coordinates {
    instructions
        .iter()
        .fold(
            (coordinates::ORIGIN, waypoint),
            |(position, waypoint), instruction| instruction.move_waypoint(position, waypoint),
        )
        .0
}

#[cfg(test)]
mod tests {
    use helpers::parse_input;

    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn test() {
            let instructions =
                parse_input::<InputRecord>(vec!["F10", "N3", "F7", "R90", "F11"]).unwrap();

            assert_eq!(travel(&instructions).manhattan_distance(), 25);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn test() {
            let instructions =
                parse_input::<InputRecord>(vec!["F10", "N3", "F7", "R90", "F11"]).unwrap();

            let waypoint = Coordinates(10, 1);

            assert_eq!(
                travel_with_waypoint(&instructions, waypoint).manhattan_distance(),
                286,
            );
        }
    }
}
