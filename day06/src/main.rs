extern crate helpers;

use std::env;
use std::process::exit;

use helpers::{handle_result, read_multiline_input};
use input_record::InputRecord;

mod input_record;

/// https://adventofcode.com/2020/day/6
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <input file> ", &args[0]);
        exit(1);
    }

    let path = &args[1];
    let values: Vec<InputRecord> = handle_result(read_multiline_input(path));

    let mut sum_anyone = 0;
    let mut sum_everyone = 0;
    for value in values {
        sum_anyone += value.nr_of_questions_anyone_answered_with_yes();
        sum_everyone += value.nr_of_questions_everyone_answered_with_yes();
    }
    println!("Part 1: {}", sum_anyone);
    println!("Part 2: {}", sum_everyone);
}

#[cfg(test)]
mod tests {
    use helpers::parse_multiline_input;

    use super::*;

    #[test]
    fn test_part_1() {
        let values = parse_multiline_input::<InputRecord>(vec![
            "abc",
            "",
            "a",
            "b",
            "c",
            "",
            "ab",
            "ac",
            "",
            "a",
            "a",
            "a",
            "a",
            "",
            "b",
        ]).unwrap();

        assert_eq!(values[0].nr_of_questions_anyone_answered_with_yes(), 3);
        assert_eq!(values[1].nr_of_questions_anyone_answered_with_yes(), 3);
        assert_eq!(values[2].nr_of_questions_anyone_answered_with_yes(), 3);
        assert_eq!(values[3].nr_of_questions_anyone_answered_with_yes(), 1);
        assert_eq!(values[4].nr_of_questions_anyone_answered_with_yes(), 1);
    }

    #[test]
    fn test_part_2() {
        let values = parse_multiline_input::<InputRecord>(vec![
            "abc",
            "",
            "a",
            "b",
            "c",
            "",
            "ab",
            "ac",
            "",
            "a",
            "a",
            "a",
            "a",
            "",
            "b",
        ]).unwrap();

        assert_eq!(values[0].nr_of_questions_everyone_answered_with_yes(), 3);
        assert_eq!(values[1].nr_of_questions_everyone_answered_with_yes(), 0);
        assert_eq!(values[2].nr_of_questions_everyone_answered_with_yes(), 1);
        assert_eq!(values[3].nr_of_questions_everyone_answered_with_yes(), 1);
        assert_eq!(values[4].nr_of_questions_everyone_answered_with_yes(), 1);
    }
}
