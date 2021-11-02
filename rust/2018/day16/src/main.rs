use std::collections::{HashMap, HashSet};

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;
use rdcl_aoc_helpers::machine::instruction::MachineInstruction;
use rdcl_aoc_helpers::machine::output_receiver::NoopOutputReceiver;
use rdcl_aoc_helpers::machine::register::{HashMapRegister, MachineRegister};

use crate::op_code::{all_op_codes, OpCode};
use crate::parse::parse_input;
use crate::sample::Sample;

mod op_code;
mod parse;
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

fn determine_op_codes(samples: &[Sample]) -> (i64, [OpCode; 16]) {
    let mut part1_count = 0;
    let mut op_codes = [OpCode::Unknown; 16];
    let mut candidates_map: HashMap<i64, HashSet<OpCode>> = HashMap::new();
    let mut decoded: HashSet<OpCode> = HashSet::with_capacity(16);

    // create a register that can be used in the steps below
    let mut register = HashMapRegister::new();

    // eliminate any candidates for which the output does not match
    for &sample in samples {
        let mut next_candidates = HashSet::new();
        for candidate in all_op_codes() {
            if candidate.test(&sample, &mut register) {
                next_candidates.insert(candidate);
            }
        }
        candidates_map.insert(sample.instruction[0], next_candidates.clone());
        if next_candidates.len() == 1 {
            let op_code = *next_candidates.iter().next().unwrap();
            op_codes[sample.instruction[0] as usize] = op_code;
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
            let i = i as i64;
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

fn run_program(instructions: &[[i64; 4]], op_codes: [OpCode; 16]) -> i64 {
    let mut register = HashMapRegister::new();
    for instr in instructions {
        let op_code = op_codes[instr[0] as usize];
        if let Some(instruction) = op_code.as_instruction(instr[1], instr[2], instr[3]) {
            instruction.execute(&mut register, &mut NoopOutputReceiver);
        } else {
            unreachable!();
        }
    }
    register.read('a')
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

        let mut register = HashMapRegister::new();

        assert!(OpCode::Mulr.test(&sample, &mut register));
        assert!(OpCode::Addi.test(&sample, &mut register));
        assert!(OpCode::Seti.test(&sample, &mut register));

        assert!(!OpCode::Addr.test(&sample, &mut register));
        assert!(!OpCode::Muli.test(&sample, &mut register));
        assert!(!OpCode::Banr.test(&sample, &mut register));
        assert!(!OpCode::Bani.test(&sample, &mut register));
        assert!(!OpCode::Borr.test(&sample, &mut register));
        assert!(!OpCode::Bori.test(&sample, &mut register));
        assert!(!OpCode::Setr.test(&sample, &mut register));
        assert!(!OpCode::Gtir.test(&sample, &mut register));
        assert!(!OpCode::Gtri.test(&sample, &mut register));
        assert!(!OpCode::Gtrr.test(&sample, &mut register));
        assert!(!OpCode::Eqir.test(&sample, &mut register));
        assert!(!OpCode::Eqri.test(&sample, &mut register));
        assert!(!OpCode::Eqrr.test(&sample, &mut register));
    }
}
