use std::collections::{HashMap, HashSet};

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;

use crate::instruction::Instruction;
use crate::op_code::{all_op_codes, OpCode};
use crate::parse::parse_input;
use crate::register::Register;
use crate::sample::Sample;

mod instruction;
mod op_code;
mod parse;
mod register;
mod sample;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let (samples, instructions) = parse_input(&args[1]).or_exit_with(1);

    let (part1_count, op_codes) = determine_op_codes(&samples);
    println!(
        "{} out of {} samples behave like three or more opcodes",
        part1_count,
        samples.len()
    );

    let output = run_program(&instructions, op_codes);
    println!("The program outputs {}.", output);
}

fn determine_op_codes(samples: &[Sample]) -> (usize, [OpCode; 16]) {
    let mut part1_count = 0;
    let mut op_codes = [OpCode::Unknown; 16];
    let mut candidates_map: HashMap<usize, HashSet<OpCode>> = HashMap::new();
    let mut decoded: HashSet<OpCode> = HashSet::with_capacity(16);

    // create a register that can be used in the steps below
    let mut register = Register::new();

    // eliminate any candidates for which the output does not match
    for &sample in samples {
        let mut next_candidates = HashSet::new();
        for candidate in all_op_codes() {
            if Instruction::test_sample(candidate, &sample, &mut register) {
                next_candidates.insert(candidate);
            }
        }
        candidates_map.insert(sample.instruction[0], next_candidates.clone());
        if next_candidates.len() == 1 {
            let op_code = *next_candidates.iter().next().unwrap();
            op_codes[sample.instruction[0]] = op_code;
            decoded.insert(op_code);
        } else if next_candidates.len() >= 3 {
            part1_count += 1;
        }
    }

    // keep trying to deduce new op codes by eliminating ones that have been decoded
    let mut found_new_op_codes = true;
    while found_new_op_codes {
        found_new_op_codes = false;
        for (i, op_code) in op_codes.iter_mut().enumerate() {
            if op_code.is_unknown() {
                let candidates = candidates_map.get_mut(&i).unwrap();
                for op_code in &decoded {
                    candidates.remove(op_code);
                }
                if candidates.len() == 1 {
                    *op_code = *candidates.iter().next().unwrap();
                    decoded.insert(*op_code);
                    found_new_op_codes = true;
                }
            }
        }
    }

    (part1_count, op_codes)
}

fn run_program(instructions: &[[usize; 4]], op_codes: [OpCode; 16]) -> usize {
    let mut register = Register::new();
    for instr in instructions {
        let op_code = op_codes[instr[0]];
        if let Some(instruction) = Instruction::from(op_code, instr[1], instr[2], instr[3]) {
            instruction.run(&mut register);
        } else {
            unreachable!();
        }
    }
    register.get(&0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let sample = Sample {
            before: [3, 2, 1, 1],
            after: [3, 2, 2, 1],
            instruction: [9, 2, 1, 2],
        };

        let mut register = Register::new();

        assert!(Instruction::test_sample(
            OpCode::Mulr,
            &sample,
            &mut register,
        ));
        assert!(Instruction::test_sample(
            OpCode::Addi,
            &sample,
            &mut register,
        ));
        assert!(Instruction::test_sample(
            OpCode::Seti,
            &sample,
            &mut register,
        ));

        assert!(!Instruction::test_sample(
            OpCode::Addr,
            &sample,
            &mut register,
        ));
        assert!(!Instruction::test_sample(
            OpCode::Muli,
            &sample,
            &mut register,
        ));
        assert!(!Instruction::test_sample(
            OpCode::Banr,
            &sample,
            &mut register,
        ));
        assert!(!Instruction::test_sample(
            OpCode::Bani,
            &sample,
            &mut register,
        ));
        assert!(!Instruction::test_sample(
            OpCode::Borr,
            &sample,
            &mut register,
        ));
        assert!(!Instruction::test_sample(
            OpCode::Bori,
            &sample,
            &mut register,
        ));
        assert!(!Instruction::test_sample(
            OpCode::Setr,
            &sample,
            &mut register,
        ));
        assert!(!Instruction::test_sample(
            OpCode::Gtir,
            &sample,
            &mut register,
        ));
        assert!(!Instruction::test_sample(
            OpCode::Gtri,
            &sample,
            &mut register,
        ));
        assert!(!Instruction::test_sample(
            OpCode::Gtrr,
            &sample,
            &mut register,
        ));
        assert!(!Instruction::test_sample(
            OpCode::Eqir,
            &sample,
            &mut register,
        ));
        assert!(!Instruction::test_sample(
            OpCode::Eqri,
            &sample,
            &mut register,
        ));
        assert!(!Instruction::test_sample(
            OpCode::Eqrr,
            &sample,
            &mut register,
        ));
    }
}
