use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::input::WithReadLines;

use scanner::Scanner;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let scanners = File::open(&args[1]).read_lines(1).collect::<Vec<Scanner>>();

    println!(
        "If you were to just leave immediately, the severity would be {}.",
        make_a_run_for_it(&scanners, 0).1
    );
    println!(
        "If you wait for {} picoseconds, you will not get caught.",
        find_minimum_delay(&scanners)
    );
}

fn make_a_run_for_it(scanners: &[Scanner], delay: u64) -> (usize, u64) {
    scanners
        .iter()
        .filter(|scanner| scanner.will_detect_you(scanner.position(delay)))
        .map(|scanner| scanner.severity())
        .fold((0, 0), |acc, severity| (acc.0 + 1, acc.1 + severity))
}

fn find_minimum_delay(scanners: &[Scanner]) -> u64 {
    (1..)
        .map(|delay| (delay, make_a_run_for_it(scanners, delay)))
        .find(|(_, (caught, _))| *caught == 0)
        .map(|(delay, _)| delay)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_a_run_for_it() {
        let scanners = vec![
            Scanner::new(0, 3),
            Scanner::new(1, 2),
            Scanner::new(4, 4),
            Scanner::new(6, 4),
        ];

        assert_eq!(make_a_run_for_it(&scanners, 0), (2, 24));
    }

    #[test]
    fn test_find_minimum_delay() {
        let scanners = vec![
            Scanner::new(0, 3),
            Scanner::new(1, 2),
            Scanner::new(4, 4),
            Scanner::new(6, 4),
        ];

        assert_eq!(find_minimum_delay(&scanners), 10);
    }
}
