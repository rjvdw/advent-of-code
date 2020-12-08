extern crate helpers;

use std::env;
use std::process::exit;

use helpers::{handle_result, read_input};
use input_record::InputRecord;

use crate::input_record::Operation;

mod input_record;

/// https://adventofcode.com/2020/day/8
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <input file>", &args[0]);
        exit(1);
    }

    let path = &args[1];
    let instructions: Vec<InputRecord> = handle_result(read_input(path));

    let (terminated, acc) = solve(&instructions);
    println!("Terminated correctly: {}, final value: {}", terminated, acc);

    match fix_program(&instructions) {
        Some((i, acc)) => println!("Terminated correctly by altering instruction at {}, final value: {}", i, acc),
        None => println!("Unable to fix program"),
    }
}

fn solve(instructions: &Vec<InputRecord>) -> (bool, i32) {
    let mut seen: Vec<bool> = Vec::with_capacity(instructions.len());
    let mut acc = 0;
    let mut idx = 0;

    for _ in instructions {
        seen.push(false);
    }

    while idx < instructions.len() {
        if seen[idx] {
            return (false, acc);
        }

        seen[idx] = true;
        let instruction = &instructions[idx];

        match instruction.op {
            Operation::ACC => {
                acc += instruction.value;
                idx += 1;
            }
            Operation::JMP => {
                idx = jump(idx, instruction.value);
            }
            Operation::NOP => {
                idx += 1;
            }
        }
    }

    (true, acc)
}

fn fix_program(instructions: &Vec<InputRecord>) -> Option<(usize, i32)> {
    for (i, instruction) in instructions.iter().enumerate() {
        let (terminated, acc) = match instruction.op {
            Operation::NOP => {
                let mut altered = instructions.clone();
                altered[i] = InputRecord { op: Operation::JMP, value: instruction.value };
                solve(&altered)
            }
            Operation::JMP => {
                let mut altered = instructions.clone();
                altered[i] = InputRecord { op: Operation::NOP, value: instruction.value };
                solve(&altered)
            }
            _ => (false, 0),
        };

        if terminated {
            return Some((i, acc));
        }
    }

    None
}

fn jump(idx: usize, value: i32) -> usize {
    if value >= 0 {
        idx + (value as usize)
    } else {
        idx - ((-value) as usize)
    }
}

#[cfg(test)]
mod tests {
    use helpers::parse_input;

    use super::*;

    #[test]
    fn test_part_1() {
        let values = parse_input::<InputRecord>(vec![
            "nop +0",
            "acc +1",
            "jmp +4",
            "acc +3",
            "jmp -3",
            "acc -99",
            "acc +1",
            "jmp -4",
            "acc +6",
        ]).unwrap();

        assert_eq!(solve(&values), (false, 5));
    }

    #[test]
    fn test_part_2() {
        let values = parse_input::<InputRecord>(vec![
            "nop +0",
            "acc +1",
            "jmp +4",
            "acc +3",
            "jmp -3",
            "acc -99",
            "acc +1",
            "jmp -4",
            "acc +6",
        ]).unwrap();

        assert_eq!(fix_program(&values), Some((7, 8)));
    }
}
