use std::collections::HashMap;
use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;
use rdcl_aoc_helpers::input::WithReadLines;

use shared::instruction::Instruction;
use shared::output_receiver::NoopOutputReceiver;
use shared::program::execute;

fn main() {
    let args = get_args(&["<input file>", "<plain eggs>", "<colored eggs>"], 1);
    let instructions = File::open(&args[1])
        .read_lines(1)
        .collect::<Vec<Instruction>>();
    let plain_eggs = args[2].parse().or_exit_with(1);
    let colored_eggs = args[3].parse().or_exit_with(1);

    let mut registers = HashMap::new();
    registers.insert('a', plain_eggs);
    execute(&instructions, &mut registers, &mut NoopOutputReceiver);
    println!(
        "Starting with {}, the value at register a is: {}",
        plain_eggs,
        registers.get(&'a').unwrap_or(&0)
    );

    let mut registers = HashMap::new();
    registers.insert('a', colored_eggs);
    execute(&instructions, &mut registers, &mut NoopOutputReceiver);
    println!(
        "Starting with {}, the value at register a is: {}",
        colored_eggs,
        registers.get(&'a').unwrap_or(&0)
    );
}

#[cfg(test)]
mod tests {
    use rdcl_aoc_helpers::input::WithAsRecords;

    use super::*;

    #[test]
    fn test_execute() {
        let instructions = vec![
            "cpy 2 a", "tgl a", "tgl a", "tgl a", "cpy 1 a", "dec a", "dec a",
        ]
        .as_records::<Instruction>()
        .unwrap();

        let mut registers = HashMap::new();
        registers.insert('a', 7);
        execute(&instructions, &mut registers, &mut NoopOutputReceiver);

        assert_eq!(*registers.get(&'a').unwrap_or(&0), 3);
    }
}
