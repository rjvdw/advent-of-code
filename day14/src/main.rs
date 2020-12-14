extern crate helpers;

use std::env;
use std::process::exit;

use helpers::{handle_result, read_input};

use crate::instruction::Instruction;
use std::collections::HashMap;

mod constants;
mod instruction;
mod v1;
mod v2;

/// https://adventofcode.com/2020/day/14
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <input file>", &args[0]);
        exit(1);
    }

    let instructions: Vec<Instruction> = handle_result(read_input(&args[1]));

    let mut memory: HashMap<usize, u64> = HashMap::new();
    match v1::run_program(&instructions, &mut memory) {
        Ok(()) => println!(
            "[v1] Sum of values in memory: {}",
            memory.values().sum::<u64>()
        ),
        Err(()) => eprintln!("[v1] Program failed to run correctly."),
    }

    let mut memory: HashMap<usize, u64> = HashMap::new();
    match v2::run_program(&instructions, &mut memory) {
        Ok(()) => println!(
            "[v2] Sum of values in memory: {}",
            memory.values().sum::<u64>(),
        ),
        Err(()) => eprintln!("[v2] Program failed to run correctly."),
    }
}

#[cfg(test)]
mod tests {
    use helpers::parse_input;

    use super::*;

    #[test]
    fn test_part_1() {
        let instructions = parse_input::<Instruction>(vec![
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X",
            "mem[8] = 11",
            "mem[7] = 101",
            "mem[8] = 0",
        ])
        .unwrap();
        let mut memory: HashMap<usize, u64> = HashMap::new();

        assert_eq!(v1::run_program(&instructions, &mut memory), Ok(()));
        assert_eq!(memory.values().sum::<u64>(), 165);
    }

    #[test]
    fn test_part_2() {
        let instructions = parse_input::<Instruction>(vec![
            "mask = 000000000000000000000000000000X1001X",
            "mem[42] = 100",
            "mask = 00000000000000000000000000000000X0XX",
            "mem[26] = 1",
        ])
        .unwrap();
        let mut memory: HashMap<usize, u64> = HashMap::new();

        assert_eq!(v2::run_program(&instructions, &mut memory), Ok(()));
        assert_eq!(memory.values().sum::<u64>(), 208);
    }
}
