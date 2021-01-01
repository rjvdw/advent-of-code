use std::collections::HashMap;
use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::input::WithReadLines;

use shared::instruction::Instruction;
use shared::output_receiver::NoopOutputReceiver;
use shared::program::execute;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let instructions = File::open(&args[1])
        .read_lines(1)
        .collect::<Vec<Instruction>>();

    let mut registers = HashMap::new();
    execute(&instructions, &mut registers, &mut NoopOutputReceiver);
    println!(
        "The value at register a is: {}",
        registers.get(&'a').unwrap_or(&0)
    );

    let mut registers = HashMap::new();
    registers.insert('c', 1);
    execute(&instructions, &mut registers, &mut NoopOutputReceiver);
    println!(
        "If we first set register c to 1, then the value at register a is: {}",
        registers.get(&'a').unwrap_or(&0)
    );
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
        execute(&instructions, &mut registers, &mut NoopOutputReceiver);

        assert_eq!(*registers.get(&'a').unwrap_or(&0), 42);
    }
}
