use std::collections::HashMap;
use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;
use rdcl_aoc_helpers::input::{MappedLines, WithReadLines};

use crate::instruction::Instruction;
use crate::signal::Signal;

mod instruction;
mod signal;

fn main() {
    let args = get_args(&["<input file>", "<wire>", "<override wire>"], 1);
    let instructions = File::open(&args[1]).read_lines::<Instruction>(1);
    let wire = args[2].parse::<Signal>().or_exit_with(1);
    let override_wire = args[3].parse::<Signal>().or_exit_with(1);
    let mut instructions_map = create_map(instructions);

    match evaluate_wire(&instructions_map, wire) {
        Some(signal) => {
            println!("The signal on {} is {}.", wire, signal);

            instructions_map.insert(
                override_wire,
                Instruction::Assign(Signal::Value(signal), override_wire),
            );
            match evaluate_wire(&instructions_map, wire) {
                Some(signal) => println!(
                    "After overriding {} with the signal from {}, the signal on {} is {}.",
                    override_wire, wire, wire, signal
                ),
                None => println!(
                    "After overriding {} with the signal from {}, there is no signal on {}.",
                    override_wire, wire, wire
                ),
            }
        }
        None => println!("There is no signal on {}.", wire),
    };
}

fn create_map(instructions: MappedLines<Instruction, File>) -> HashMap<Signal, Instruction> {
    let mut map = HashMap::new();
    for instruction in instructions {
        map.insert(instruction.get_output(), instruction);
    }
    map
}

fn evaluate_wire(map: &HashMap<Signal, Instruction>, wire: Signal) -> Option<u16> {
    let mut evaluated: HashMap<Signal, u16> = HashMap::new();
    map.get(&wire)?.evaluate(map, &mut evaluated)
}

#[cfg(test)]
mod tests {
    use rdcl_aoc_helpers::input::WithAsRecords;

    use super::*;

    #[test]
    fn test_wire_d() {
        let instructions_map = get_instructions();
        let result = evaluate_wire(&instructions_map, "d".parse::<Signal>().unwrap());

        assert_eq!(result, Some(72));
    }

    #[test]
    fn test_wire_e() {
        let instructions_map = get_instructions();
        let result = evaluate_wire(&instructions_map, "e".parse::<Signal>().unwrap());

        assert_eq!(result, Some(507));
    }

    #[test]
    fn test_wire_f() {
        let instructions_map = get_instructions();
        let result = evaluate_wire(&instructions_map, "f".parse::<Signal>().unwrap());

        assert_eq!(result, Some(492));
    }

    #[test]
    fn test_wire_g() {
        let instructions_map = get_instructions();
        let result = evaluate_wire(&instructions_map, "g".parse::<Signal>().unwrap());

        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_wire_h() {
        let instructions_map = get_instructions();
        let result = evaluate_wire(&instructions_map, "h".parse::<Signal>().unwrap());

        assert_eq!(result, Some(65412));
    }

    #[test]
    fn test_wire_i() {
        let instructions_map = get_instructions();
        let result = evaluate_wire(&instructions_map, "i".parse::<Signal>().unwrap());

        assert_eq!(result, Some(65079));
    }

    #[test]
    fn test_wire_x() {
        let instructions_map = get_instructions();
        let result = evaluate_wire(&instructions_map, "x".parse::<Signal>().unwrap());

        assert_eq!(result, Some(123));
    }

    #[test]
    fn test_wire_y() {
        let instructions_map = get_instructions();
        let result = evaluate_wire(&instructions_map, "y".parse::<Signal>().unwrap());

        assert_eq!(result, Some(456));
    }

    fn get_instructions() -> HashMap<Signal, Instruction> {
        let instructions = vec![
            "123 -> x",
            "456 -> y",
            "x AND y -> d",
            "x OR y -> e",
            "x LSHIFT 2 -> f",
            "y RSHIFT 2 -> g",
            "NOT x -> h",
            "NOT y -> i",
        ]
        .as_records::<Instruction>()
        .unwrap();

        let mut map = HashMap::new();
        for instruction in instructions {
            map.insert(instruction.get_output(), instruction);
        }
        map
    }
}
