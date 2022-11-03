extern crate rdcl_aoc_helpers;

use std::fs::File;
use std::io::{BufRead, BufReader};

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;

use crate::analyze_instructions::analyze_instructions;
use crate::sub_routine::SubRoutine;

mod analyze_instructions;
mod sub_routine;

const DIGITS_INCR: [i64; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
const DIGITS_DECR: [i64; 9] = [9, 8, 7, 6, 5, 4, 3, 2, 1];

fn main() {
    let args = get_args(&["<input file>"], 1);

    let file = File::open(&args[1]).or_exit_with(1);
    let reader = BufReader::new(file).lines();
    let sub_routines = analyze_instructions(reader).or_exit_with(1);

    match find_model_number(&sub_routines, Mode::Largest) {
        Some(model_number) => println!("The largest valid model number is {}.", model_number),
        None => eprintln!("There are no valid model numbers."),
    }

    match find_model_number(&sub_routines, Mode::Smallest) {
        Some(model_number) => println!("The smallest valid model number is {}.", model_number),
        None => eprintln!("There are no valid model numbers."),
    }
}

enum Mode {
    Largest,
    Smallest,
}

impl Mode {
    fn get_digits(&self) -> [i64; 9] {
        match self {
            Mode::Largest => DIGITS_INCR,
            Mode::Smallest => DIGITS_DECR,
        }
    }
}

/// There are two types of subroutines. The ones with divides=false will always grow the z value
/// with a factor of ~26. These subroutines will _always_ fulfill the condition `x != w`. The
/// subroutines with divides=true will shrink the z value when `x == w` and will grow the z value
/// with some constant otherwise.
///
/// Since in the puzzle input there are an equal number of subroutines with divides=false and
/// divides=true, this means that for _every_ subroutine with divides=true, the z value must shrink.
/// Otherwise the z value will never reach 0.
///
/// This means that if we start evaluating from left to right, everytime we encounter a subroutine
/// with divides=true, there can only be one digit that is valid (because it must be equal to x). If
/// x < 1 or x > 9, then there is no valid digit.
fn find_model_number(sub_routines: &[SubRoutine], mode: Mode) -> Option<i64> {
    let mut stack = vec![(0, 0, 0)];

    while let Some((number, idx, z)) = stack.pop() {
        if idx >= sub_routines.len() {
            // we are done. if this number is valid, then it must be the biggest number
            if z == 0 {
                return Some(number);
            } else {
                continue;
            }
        }

        let sr = &sub_routines[idx];

        if sr.divides {
            let x = sr.get_x_value(z);
            if (1..10).contains(&x) {
                let number = 10 * number + x;
                let z = sr.run(x, z);
                stack.push((number, idx + 1, z));
            } else {
                // invalid, ignore this branch
                continue;
            }
        } else {
            for w in mode.get_digits() {
                let number = 10 * number + w;
                let z = sr.run(w, z);
                stack.push((number, idx + 1, z));
            }
        }
    }

    None
}
