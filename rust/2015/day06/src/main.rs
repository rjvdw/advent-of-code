use std::collections::{HashMap, HashSet};
use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::input::{MappedLines, WithReadLines};

use crate::instruction::{Coordinate, Instruction};

mod instruction;

fn main() {
    let args = get_args(&["<input file>"], 1);

    let instructions = File::open(&args[1]).read_lines::<Instruction>(1);
    let (lit, brightness) = follow_instructions(instructions);

    println!(
        "After following the instructions, there are {} lights turned on.",
        lit.len()
    );
    println!(
        "Following the instructions correctly, the total brightness is {}.",
        brightness.values().cloned().sum::<u32>()
    );
}

fn follow_instructions(
    instructions: MappedLines<Instruction, File>,
) -> (HashSet<Coordinate>, HashMap<Coordinate, u32>) {
    let mut lit = HashSet::new();
    let mut brightness = HashMap::new();
    for instruction in instructions {
        instruction.set_binary_state(&mut lit);
        instruction.update_brightness(&mut brightness);
    }
    (lit, brightness)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let instruction = "turn on 0,0 through 999,999"
            .parse::<Instruction>()
            .unwrap();
        assert_eq!(instruction.count_lights(), 1_000_000);
    }

    #[test]
    fn test_2() {
        let instruction = "toggle 0,0 through 999,0".parse::<Instruction>().unwrap();
        assert_eq!(instruction.count_lights(), 1000);
    }

    #[test]
    fn test_3() {
        let instruction = "turn off 499,499 through 500,500"
            .parse::<Instruction>()
            .unwrap();
        assert_eq!(instruction.count_lights(), 4);
    }
}
