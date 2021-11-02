use std::collections::HashMap;
use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::input::WithReadLines;

use crate::instruction::Instruction;

mod condition;
mod instruction;
mod operation;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let instructions = File::open(&args[1])
        .read_lines(1)
        .collect::<Vec<Instruction>>();

    let (highest_value_end, highest_value_ever) = run_program(&instructions);
    println!(
        "The highest value in any register is {}.",
        highest_value_end
    );
    println!(
        "The highest value ever held in any register is {}.",
        highest_value_ever
    );
}

fn run_program(instructions: &[Instruction]) -> (i64, i64) {
    let mut highest_value_ever = i64::MIN;
    let mut register = HashMap::new();
    for instruction in instructions {
        instruction.run(&mut register);
        let hv = *register.values().max().unwrap_or(&0);
        if hv > highest_value_ever {
            highest_value_ever = hv;
        }
    }
    (*register.values().max().unwrap_or(&0), highest_value_ever)
}

#[cfg(test)]
mod tests {
    use rdcl_aoc_helpers::input::WithAsRecords;

    use super::*;

    #[test]
    fn test_run_program() {
        let instructions = vec![
            "b inc 5 if a > 1",
            "a inc 1 if b < 5",
            "c dec -10 if a >= 1",
            "c inc -20 if c == 10",
        ]
        .as_records::<Instruction>()
        .unwrap();

        assert_eq!(run_program(&instructions), (1, 10));
    }
}
