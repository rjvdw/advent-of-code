extern crate rdcl_aoc_helpers;

use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;
use rdcl_aoc_helpers::input::WithReadLines;

use crate::coordinates::Coordinates;
use crate::instruction::Instruction;

mod coordinates;
mod instruction;

fn main() {
    let args = get_args(&["<input file>", "<waypoint x>,<waypoint y>"], 1);

    let instructions = File::open(&args[1])
        .read_lines(1)
        .collect::<Vec<Instruction>>();
    let waypoint = args[2].parse::<Coordinates>().or_exit_with(1);

    println!(
        "Manhattan distance without using waypoint: {}",
        travel(&instructions).manhattan_distance(),
    );

    println!(
        "Manhattan distance when using waypoint: {}",
        travel_with_waypoint(&instructions, waypoint).manhattan_distance(),
    );
}

fn travel(instructions: &[Instruction]) -> Coordinates {
    instructions
        .iter()
        .fold(
            (coordinates::ORIGIN, 0),
            |(position, heading), instruction| instruction.travel(position, heading),
        )
        .0
}

fn travel_with_waypoint(instructions: &[Instruction], waypoint: Coordinates) -> Coordinates {
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
    use rdcl_aoc_helpers::input::WithAsRecords;

    use super::*;

    #[test]
    fn test_travel() {
        let instructions = vec!["F10", "N3", "F7", "R90", "F11"]
            .as_records::<Instruction>()
            .unwrap();
        assert_eq!(travel(&instructions).manhattan_distance(), 25);
    }

    #[test]
    fn test_travel_with_waypoint() {
        let instructions = vec!["F10", "N3", "F7", "R90", "F11"]
            .as_records::<Instruction>()
            .unwrap();
        let waypoint = Coordinates(10, 1);

        assert_eq!(
            travel_with_waypoint(&instructions, waypoint).manhattan_distance(),
            286,
        );
    }
}
