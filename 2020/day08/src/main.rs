extern crate helpers;

use std::env;
use std::process::exit;

use helpers::handle_result;
use helpers::read::read_input;
use input_record::InputRecord;

use crate::input_record::Operation;
use crate::program_fixer::ProgramFixer;

mod input_record;
mod program_fixer;

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
    if terminated {
        println!("Program terminated correctly. Final value is {}", acc);
    } else {
        println!(
            "Program did not terminate correctly. Final value is {}",
            acc
        );
        let fixer = ProgramFixer::new(&instructions, solve);
        for (i, acc) in fixer {
            println!(
                "Terminated correctly by altering instruction at line {}, final value is {}",
                i + 1,
                acc
            );
        }
    }
}

fn solve(instructions: &[InputRecord]) -> (bool, i32) {
    let mut seen: Vec<bool> = Vec::with_capacity(instructions.len());
    let mut acc = 0;
    let mut idx = 0;
    seen.resize(instructions.len(), false);

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

fn jump(idx: usize, value: i32) -> usize {
    if value >= 0 {
        idx + (value as usize)
    } else {
        idx - ((-value) as usize)
    }
}

#[cfg(test)]
mod tests {
    use helpers::parse::parse_input;

    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn test() {
            let values = parse_input::<InputRecord>(vec![
                "nop +0", "acc +1", "jmp +4", "acc +3", "jmp -3", "acc -99", "acc +1", "jmp -4",
                "acc +6",
            ])
            .unwrap();

            assert_eq!(solve(&values), (false, 5));
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn test() {
            let values = parse_input::<InputRecord>(vec![
                "nop +0", "acc +1", "jmp +4", "acc +3", "jmp -3", "acc -99", "acc +1", "jmp -4",
                "acc +6",
            ])
            .unwrap();

            let fixer = ProgramFixer::new(&values, solve);
            let fixes: Vec<(usize, i32)> = fixer.collect();

            assert_eq!(fixes.len(), 1);
            assert_eq!(fixes[0], (7, 8));
        }
    }
}
