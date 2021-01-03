use std::collections::HashSet;
use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::input::WithReadLines;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let changes = File::open(&args[1]).read_lines(1).collect::<Vec<i32>>();

    let frequency = apply_changes(&changes, 0);
    println!("The frequency after applying the changes is {}.", frequency);
    let frequency = find_repeating_frequency(&changes, 0);
    println!("The first frequency that repeats is {}.", frequency);
}

fn apply_changes(changes: &[i32], mut frequency: i32) -> i32 {
    for change in changes {
        frequency += change;
    }
    frequency
}

fn find_repeating_frequency(changes: &[i32], mut frequency: i32) -> i32 {
    let mut seen = HashSet::new();
    seen.insert(frequency);
    loop {
        for change in changes {
            frequency += change;
            if seen.contains(&frequency) {
                return frequency;
            }
            seen.insert(frequency);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_changes_1() {
        assert_eq!(apply_changes(&[1, -2, 3, 1], 0), 3);
    }

    #[test]
    fn test_apply_changes_2() {
        assert_eq!(apply_changes(&[1, 1, 1], 0), 3);
    }

    #[test]
    fn test_apply_changes_3() {
        assert_eq!(apply_changes(&[1, 1, -2], 0), 0);
    }

    #[test]
    fn test_apply_changes_4() {
        assert_eq!(apply_changes(&[-1, -2, -3], 0), -6);
    }

    #[test]
    fn test_find_repeating_frequency_1() {
        assert_eq!(find_repeating_frequency(&[1, -2, 3, 1], 0), 2);
    }

    #[test]
    fn test_find_repeating_frequency_2() {
        assert_eq!(find_repeating_frequency(&[1, -1], 0), 0);
    }

    #[test]
    fn test_find_repeating_frequency_3() {
        assert_eq!(find_repeating_frequency(&[3, 3, 4, -2, -4], 0), 10);
    }

    #[test]
    fn test_find_repeating_frequency_4() {
        assert_eq!(find_repeating_frequency(&[-6, 3, 8, 5, -6], 0), 5);
    }

    #[test]
    fn test_find_repeating_frequency_5() {
        assert_eq!(find_repeating_frequency(&[7, 7, -2, -7, -4], 0), 14);
    }
}
