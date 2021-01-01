use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;
use rdcl_aoc_helpers::input::WithReadLines;

use crate::instruction::{Instruction, Value};

mod instruction;

fn main() {
    let args = get_args(&["<input file>", "<plain eggs>", "<colored eggs>"], 1);
    let instructions = File::open(&args[1])
        .read_lines(1)
        .collect::<Vec<Instruction>>();
    let plain_eggs = args[2].parse().or_exit_with(1);
    let colored_eggs = args[3].parse().or_exit_with(1);

    let mut registers = HashMap::new();
    registers.insert('a', plain_eggs);
    execute(&instructions, &mut registers);
    println!(
        "Starting with {}, the value at register a is: {}",
        plain_eggs,
        registers.get(&'a').unwrap_or(&0)
    );

    let mut registers = HashMap::new();
    registers.insert('a', colored_eggs);
    execute(&instructions, &mut registers);
    println!(
        "Starting with {}, the value at register a is: {}",
        colored_eggs,
        registers.get(&'a').unwrap_or(&0)
    );
}

fn execute(instructions: &[Instruction], registers: &mut HashMap<char, i32>) {
    let mut instructions = instructions.to_vec();
    let mut idx = 0;
    while let Some(instruction) = safe_get(&instructions, idx) {
        if apply_optimization(&instructions, idx, registers) {
            idx += 6;
        } else {
            match instruction {
                Instruction::Toggle(_) => {
                    let target_idx = idx + instruction.run(registers);
                    let target_idx_opt = match usize::try_from(target_idx) {
                        Ok(idx) => Some(idx),
                        Err(_) => None,
                    };
                    if let Some(instruction) =
                        target_idx_opt.and_then(|idx| instructions.get_mut(idx))
                    {
                        match &instruction {
                            Instruction::Copy(a, b) => {
                                *instruction =
                                    Instruction::JumpNotZero(a.clone(), Value::Register(*b));
                            }
                            Instruction::Increment(a) => {
                                *instruction = Instruction::Decrement(*a);
                            }
                            Instruction::Decrement(a) => {
                                *instruction = Instruction::Increment(*a);
                            }
                            Instruction::JumpNotZero(a, Value::Register(b)) => {
                                *instruction = Instruction::Copy(a.clone(), *b);
                            }
                            Instruction::Toggle(Value::Register(a)) => {
                                *instruction = Instruction::Increment(*a);
                            }
                            _ => {
                                eprintln!("Cannot transform instruction '{:?}'.", instruction);
                                panic!("cannot transform instruction");
                            }
                        }
                    }
                    idx += 1;
                }
                _ => {
                    idx += instruction.run(registers);
                }
            }
        }
    }
}

fn safe_get(instructions: &[Instruction], idx: i32) -> Option<&Instruction> {
    match usize::try_from(idx) {
        Ok(idx) => instructions.get(idx),
        _ => None,
    }
}

fn apply_optimization(
    instructions: &[Instruction],
    idx: i32,
    registers: &mut HashMap<char, i32>,
) -> bool {
    // If the next 6 lines are:
    //   cpy b c
    //   inc a
    //   dec c
    //   jnz c -2
    //   dec d
    //   jnz d -5
    // we are actually doing multiplication. This can be optimized, by simply doing:
    //   * Increment register a by b*d.
    //   * Clear register c.
    //   * Clear register d.

    match usize::try_from(idx) {
        Ok(idx) => {
            if idx + 5 < instructions.len() {
                // cpy b c
                let (val_b, c) = if let Some(Instruction::Copy(a, b)) = instructions.get(idx) {
                    (a.get_value(registers), *b)
                } else {
                    return false;
                };

                // inc a
                let a = if let Some(Instruction::Increment(a)) = instructions.get(idx + 1) {
                    *a
                } else {
                    return false;
                };

                // dec c
                if let Some(Instruction::Decrement(ch)) = instructions.get(idx + 2) {
                    if *ch != c {
                        return false;
                    }
                } else {
                    return false;
                };

                // jnz c -2
                if let Some(Instruction::JumpNotZero(Value::Register(ch), Value::Raw(-2))) =
                    instructions.get(idx + 3)
                {
                    if *ch != c {
                        return false;
                    }
                } else {
                    return false;
                };

                // dec d
                let d = if let Some(Instruction::Decrement(d)) = instructions.get(idx + 4) {
                    *d
                } else {
                    return false;
                };

                // jnz d -5
                if let Some(Instruction::JumpNotZero(Value::Register(ch), Value::Raw(-5))) =
                    instructions.get(idx + 5)
                {
                    if *ch != d {
                        return false;
                    }
                } else {
                    return false;
                };

                let val_a = *registers.get(&a).unwrap_or(&0);
                let val_d = *registers.get(&d).unwrap_or(&0);

                registers.insert(a, val_a + val_b * val_d);
                registers.insert(c, 0);
                registers.insert(d, 0);

                true
            } else {
                false
            }
        }
        _ => false,
    }
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
        execute(&instructions, &mut registers);

        assert_eq!(*registers.get(&'a').unwrap_or(&0), 3);
    }
}
