#[macro_use]
extern crate lazy_static;

use std::fs::File;

use fancy_regex::Regex;
use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::input::WithReadLines;

const DISALLOWED: [(char, char); 4] = [('a', 'b'), ('c', 'd'), ('p', 'q'), ('x', 'y')];
const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn main() {
    let args = get_args(&["<input file>"], 1);

    let mut counts = (0, 0);
    for line in File::open(&args[1]).read_lines::<String>(1) {
        if is_nice_according_to_old_model(&line) {
            counts.0 += 1;
        }
        if is_nice_according_to_new_model(&line) {
            counts.1 += 1;
        }
    }

    println!(
        "There are {} lines that are nice according to the old model.",
        counts.0
    );
    println!(
        "There are {} lines that are nice according to the new model.",
        counts.1
    );
}

fn is_nice_according_to_old_model(line: &str) -> bool {
    let mut chars = line.chars();

    if let Some(first_char) = chars.next() {
        let mut nr_vowels = i32::from(VOWELS.contains(&first_char));
        let mut prev_char = first_char;
        let mut has_double_chars = false;

        for ch in chars {
            if DISALLOWED.contains(&(prev_char, ch)) {
                return false;
            }

            if VOWELS.contains(&ch) {
                nr_vowels += 1;
            }

            if prev_char == ch {
                has_double_chars = true;
            }

            prev_char = ch;
        }

        nr_vowels >= 3 && has_double_chars
    } else {
        false
    }
}

fn is_nice_according_to_new_model(line: &str) -> bool {
    lazy_static! {
        static ref RE1: Regex = Regex::new(r"(..).*\1").unwrap();
        static ref RE2: Regex = Regex::new(r"(.).\1").unwrap();
    }

    RE1.is_match(line).unwrap_or(false) && RE2.is_match(line).unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod old_model {
        use super::*;

        #[test]
        fn test_nice_string_1() {
            assert!(is_nice_according_to_old_model("ugknbfddgicrmopn"));
        }

        #[test]
        fn test_nice_string_2() {
            assert!(is_nice_according_to_old_model("aaa"));
        }

        #[test]
        fn test_naughty_string_1() {
            assert!(!is_nice_according_to_old_model("jchzalrnumimnmhp"));
        }

        #[test]
        fn test_naughty_string_2() {
            assert!(!is_nice_according_to_old_model("haegwjzuvuyypxyu"));
        }

        #[test]
        fn test_naughty_string_3() {
            assert!(!is_nice_according_to_old_model("dvszwmarrgswjxmb"));
        }
    }

    mod new_model {
        use super::*;

        #[test]
        fn test_nice_string_1() {
            assert!(is_nice_according_to_new_model("qjhvhtzxzqqjkmpb"));
        }

        #[test]
        fn test_nice_string_2() {
            assert!(is_nice_according_to_new_model("xxyxx"));
        }

        #[test]
        fn test_naughty_string_1() {
            assert!(!is_nice_according_to_new_model("uurcxstgmygtbstg"));
        }

        #[test]
        fn test_naughty_string_2() {
            assert!(!is_nice_according_to_new_model("ieodomkazucvgmuy"));
        }
    }
}
