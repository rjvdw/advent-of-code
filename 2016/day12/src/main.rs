use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::input::WithReadLines;

use crate::instruction::Instruction;

mod instruction;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let instructions = File::open(&args[1])
        .read_lines(1)
        .collect::<Vec<Instruction>>();

    let mut registers = HashMap::new();
    execute(&instructions, &mut registers);
    println!(
        "The value at register a is: {}",
        registers.get(&'a').unwrap_or(&0)
    );

    let mut registers = HashMap::new();
    registers.insert('c', 1);
    execute(&instructions, &mut registers);
    println!(
        "If we first set register c to 1, then the value at register a is: {}",
        registers.get(&'a').unwrap_or(&0)
    );
}

fn execute(instructions: &[Instruction], registers: &mut HashMap<char, i32>) {
    let mut idx = 0;
    while let Some(instruction) = safe_get(instructions, idx) {
        idx += instruction.run(registers);
    }
}

fn safe_get(instructions: &[Instruction], idx: i32) -> Option<&Instruction> {
    match usize::try_from(idx) {
        Ok(idx) => instructions.get(idx),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use rdcl_aoc_helpers::input::WithAsRecords;

    use super::*;

    #[test]
    fn test_execute() {
        let instructions = vec!["cpy 41 a", "inc a", "inc a", "dec a", "jnz a 2", "dec a"]
            .as_records::<Instruction>()
            .unwrap();

        let mut registers = HashMap::new();
        execute(&instructions, &mut registers);

        assert_eq!(*registers.get(&'a').unwrap_or(&0), 42);
    }
}
