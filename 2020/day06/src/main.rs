extern crate rdcl_aoc_helpers;

use std::env;
use std::fs::File;
use std::process::exit;

use rdcl_aoc_helpers::input::WithReadMultiLines;

use group::Group;

mod group;

/// https://adventofcode.com/2020/day/6
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <input file> ", &args[0]);
        exit(1);
    }

    let groups = File::open(&args[1]).read_multi_lines::<Group>(1);

    let mut counts = (0, 0);
    for group in groups {
        counts.0 += group.nr_of_questions_anyone_answered_with_yes();
        counts.1 += group.nr_of_questions_everyone_answered_with_yes();
    }
    println!(
        "The sum of the number of questions per group to which anyone answered yes: {}",
        counts.0
    );
    println!(
        "The sum of the number of questions per group to which everyone answered yes: {}",
        counts.1
    );
}

#[cfg(test)]
mod tests {
    use rdcl_aoc_helpers::input::WithAsMultilineRecords;

    use super::*;

    #[test]
    fn test_nr_of_questions_anyone_answered_with_yes() {
        let values = vec![
            "abc", "", "a", "b", "c", "", "ab", "ac", "", "a", "a", "a", "a", "", "b",
        ]
        .as_multiline_records::<Group>()
        .unwrap();

        assert_eq!(values[0].nr_of_questions_anyone_answered_with_yes(), 3);
        assert_eq!(values[1].nr_of_questions_anyone_answered_with_yes(), 3);
        assert_eq!(values[2].nr_of_questions_anyone_answered_with_yes(), 3);
        assert_eq!(values[3].nr_of_questions_anyone_answered_with_yes(), 1);
        assert_eq!(values[4].nr_of_questions_anyone_answered_with_yes(), 1);
    }

    #[test]
    fn test_nr_of_questions_everyone_answered_with_yes() {
        let values = vec![
            "abc", "", "a", "b", "c", "", "ab", "ac", "", "a", "a", "a", "a", "", "b",
        ]
        .as_multiline_records::<Group>()
        .unwrap();

        assert_eq!(values[0].nr_of_questions_everyone_answered_with_yes(), 3);
        assert_eq!(values[1].nr_of_questions_everyone_answered_with_yes(), 0);
        assert_eq!(values[2].nr_of_questions_everyone_answered_with_yes(), 1);
        assert_eq!(values[3].nr_of_questions_everyone_answered_with_yes(), 1);
        assert_eq!(values[4].nr_of_questions_everyone_answered_with_yes(), 1);
    }
}
