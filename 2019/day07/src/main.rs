use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;

use shared::intcode;
use shared::intcode::parse::parse_input;

use crate::amplifiers::Amplifiers;
use rdcl_aoc_helpers::permutations::Permutations;

mod amplifiers;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let file = File::open(&args[1]).or_exit_with(1);
    let program = parse_input(file).or_exit_with(1);

    println!(
        "[debugging] The highest possible output signal is {}.",
        try_phase_settings(&program, 0..=4, true)
    );

    println!(
        "The highest possible output signal is {}.",
        try_phase_settings(&program, 5..=9, false)
    );
}

fn try_phase_settings<I>(program: &intcode::Program, phase_settings: I, debugging: bool) -> i64
where
    I: Iterator<Item = i64> + Clone,
{
    let count = phase_settings.clone().count();
    let mut best = i64::MIN;
    let permutations: Permutations<i64> = phase_settings.collect();
    for phase_settings in permutations {
        let mut amplifiers = Amplifiers::new(&program, count);
        let output = amplifiers.run(&phase_settings, 0, debugging);
        if output > best {
            best = output;
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use shared::program;

    use super::*;

    #[test]
    fn test_try_phase_settings_1() {
        let program = program![3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0];
        assert_eq!(try_phase_settings(&program, 0..=4, true), 43210);
    }

    #[test]
    fn test_try_phase_settings_2() {
        let program = program![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0
        ];
        assert_eq!(try_phase_settings(&program, 0..=4, true), 54321);
    }

    #[test]
    fn test_try_phase_settings_3() {
        let program = program![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0
        ];
        assert_eq!(try_phase_settings(&program, 0..=4, true), 65210);
    }

    #[test]
    fn test_try_phase_settings_4() {
        let program = program![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5
        ];
        assert_eq!(try_phase_settings(&program, 5..=9, false), 139629729);
    }

    #[test]
    fn test_try_phase_settings_5() {
        let program = program![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10
        ];
        assert_eq!(try_phase_settings(&program, 5..=9, false), 18216);
    }
}
