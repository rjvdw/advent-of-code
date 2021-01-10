extern crate rdcl_aoc_helpers;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::parse_error;

use crate::rule::Rule;

mod rule;
mod rules_map;
mod splittable_and_parsable;

/// https://adventofcode.com/2020/day/19
fn main() {
    let args = get_args(&["<input file>"], 1);

    match read(&args[1]) {
        Ok(result) => println!("The result is: {}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn read(path: &str) -> Result<usize, ParseError> {
    let mut lines = BufReader::new(File::open(path)?).lines();

    // first collect the rules
    let mut rules: HashMap<usize, Rule> = HashMap::new();
    while let Some(line) = lines.next() {
        let line = line?;

        if line.is_empty() {
            break;
        }

        match line.find(':') {
            Some(idx) => {
                let key = line[..idx].parse::<usize>()?;
                let value = line[idx + 2..].parse::<Rule>()?;
                rules.insert(key, value);
            }
            None => return Err(parse_error!("Invalid rule encountered: {}", line)),
        }
    }

    // then check which messages match
    let mut count = 0;
    let rule0 = match rules.get(&0) {
        Some(rule) => rule,
        None => return Err(parse_error!("Rule 0 does not exist")),
    };
    for line in lines {
        if rule0.test(&rules, &line?) {
            count += 1;
        }
    }

    Ok(count)
}

#[cfg(test)]
#[rustfmt::skip::macros(assert)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let mut rules = HashMap::new();
        rules.insert(0, "1 2".parse::<Rule>().unwrap());
        rules.insert(1, "\"a\"".parse::<Rule>().unwrap());
        rules.insert(2, "1 3 | 3 1".parse::<Rule>().unwrap());
        rules.insert(3, "\"b\"".parse::<Rule>().unwrap());

        assert!(is_valid(&rules, "a", 1));
        assert!(is_valid(&rules, "ab", 2));
        assert!(is_valid(&rules, "aab", 0));
        assert!(is_valid(&rules, "aba", 0));
        assert!(!is_valid(&rules, "aaba", 0));
        assert!(!is_valid(&rules, "aabab", 0));
        assert!(!is_valid(&rules, "bbbbbbb", 0));
    }

    #[test]
    fn test_example_2() {
        let mut rules = HashMap::new();
        rules.insert(0, "4 1 5".parse::<Rule>().unwrap());
        rules.insert(1, "2 3 | 3 2".parse::<Rule>().unwrap());
        rules.insert(2, "4 4 | 5 5".parse::<Rule>().unwrap());
        rules.insert(3, "4 5 | 5 4".parse::<Rule>().unwrap());
        rules.insert(4, "\"a\"".parse::<Rule>().unwrap());
        rules.insert(5, "\"b\"".parse::<Rule>().unwrap());

        assert!(is_valid(&rules, "aaab", 1));
        assert!(is_valid(&rules, "aaba", 1));
        assert!(is_valid(&rules, "bbab", 1));
        assert!(is_valid(&rules, "bbba", 1));
        assert!(is_valid(&rules, "abaa", 1));
        assert!(is_valid(&rules, "abbb", 1));
        assert!(is_valid(&rules, "baaa", 1));
        assert!(is_valid(&rules, "babb", 1));

        assert!(is_valid(&rules, "aaaabb", 0));
        assert!(is_valid(&rules, "aaabab", 0));
        assert!(is_valid(&rules, "abbabb", 0));
        assert!(is_valid(&rules, "abbbab", 0));
        assert!(is_valid(&rules, "aabaab", 0));
        assert!(is_valid(&rules, "aabbbb", 0));
        assert!(is_valid(&rules, "abaaab", 0));
        assert!(is_valid(&rules, "ababbb", 0));
    }

    #[test]
    fn test_example_3() {
        let mut rules = HashMap::new();
        rules.insert(42, "9 14 | 10 1".parse::<Rule>().unwrap());
        rules.insert(9, "14 27 | 1 26".parse::<Rule>().unwrap());
        rules.insert(10, "23 14 | 28 1".parse::<Rule>().unwrap());
        rules.insert(1, "\"a\"".parse::<Rule>().unwrap());
        rules.insert(11, "42 31".parse::<Rule>().unwrap());
        rules.insert(5, "1 14 | 15 1".parse::<Rule>().unwrap());
        rules.insert(19, "14 1 | 14 14".parse::<Rule>().unwrap());
        rules.insert(12, "24 14 | 19 1".parse::<Rule>().unwrap());
        rules.insert(16, "15 1 | 14 14".parse::<Rule>().unwrap());
        rules.insert(31, "14 17 | 1 13".parse::<Rule>().unwrap());
        rules.insert(6, "14 14 | 1 14".parse::<Rule>().unwrap());
        rules.insert(2, "1 24 | 14 4".parse::<Rule>().unwrap());
        rules.insert(0, "8 11".parse::<Rule>().unwrap());
        rules.insert(13, "14 3 | 1 12".parse::<Rule>().unwrap());
        rules.insert(15, "1 | 14".parse::<Rule>().unwrap());
        rules.insert(17, "14 2 | 1 7".parse::<Rule>().unwrap());
        rules.insert(23, "25 1 | 22 14".parse::<Rule>().unwrap());
        rules.insert(28, "16 1".parse::<Rule>().unwrap());
        rules.insert(4, "1 1".parse::<Rule>().unwrap());
        rules.insert(20, "14 14 | 1 15".parse::<Rule>().unwrap());
        rules.insert(3, "5 14 | 16 1".parse::<Rule>().unwrap());
        rules.insert(27, "1 6 | 14 18".parse::<Rule>().unwrap());
        rules.insert(14, "\"b\"".parse::<Rule>().unwrap());
        rules.insert(21, "14 1 | 1 14".parse::<Rule>().unwrap());
        rules.insert(25, "1 1 | 1 14".parse::<Rule>().unwrap());
        rules.insert(22, "14 14".parse::<Rule>().unwrap());
        rules.insert(8, "42".parse::<Rule>().unwrap());
        rules.insert(26, "14 22 | 1 20".parse::<Rule>().unwrap());
        rules.insert(18, "15 15".parse::<Rule>().unwrap());
        rules.insert(7, "14 5 | 1 21".parse::<Rule>().unwrap());
        rules.insert(24, "14 1".parse::<Rule>().unwrap());

        assert!(!is_valid(&rules, "abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa", 0));
        assert!(is_valid(&rules, "bbabbbbaabaabba", 0));
        assert!(!is_valid(&rules, "babbbbaabbbbbabbbbbbaabaaabaaa", 0));
        assert!(!is_valid(&rules, "aaabbbbbbaaaabaababaabababbabaaabbababababaaa", 0));
        assert!(!is_valid(&rules, "bbbbbbbaaaabbbbaaabbabaaa", 0));
        assert!(!is_valid(&rules, "bbbababbbbaaaaaaaabbababaaababaabab", 0));
        assert!(is_valid(&rules, "ababaaaaaabaaab", 0));
        assert!(is_valid(&rules, "ababaaaaabbbaba", 0));
        assert!(!is_valid(&rules, "baabbaaaabbaaaababbaababb", 0));
        assert!(!is_valid(&rules, "abbbbabbbbaaaababbbbbbaaaababb", 0));
        assert!(!is_valid(&rules, "aaaaabbaabaaaaababaa", 0));
        assert!(!is_valid(&rules, "aaaabbaaaabbaaa", 0));
        assert!(!is_valid(&rules, "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa", 0));
        assert!(!is_valid(&rules, "babaaabbbaaabaababbaabababaaab", 0));
        assert!(!is_valid(&rules, "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba", 0));
    }

    #[test]
    fn test_example_4() {
        let mut rules = HashMap::new();
        rules.insert(42, "9 14 | 10 1".parse::<Rule>().unwrap());
        rules.insert(9, "14 27 | 1 26".parse::<Rule>().unwrap());
        rules.insert(10, "23 14 | 28 1".parse::<Rule>().unwrap());
        rules.insert(1, "\"a\"".parse::<Rule>().unwrap());
        rules.insert(11, "42 31 | 42 11 31".parse::<Rule>().unwrap());
        rules.insert(5, "1 14 | 15 1".parse::<Rule>().unwrap());
        rules.insert(19, "14 1 | 14 14".parse::<Rule>().unwrap());
        rules.insert(12, "24 14 | 19 1".parse::<Rule>().unwrap());
        rules.insert(16, "15 1 | 14 14".parse::<Rule>().unwrap());
        rules.insert(31, "14 17 | 1 13".parse::<Rule>().unwrap());
        rules.insert(6, "14 14 | 1 14".parse::<Rule>().unwrap());
        rules.insert(2, "1 24 | 14 4".parse::<Rule>().unwrap());
        rules.insert(0, "8 11".parse::<Rule>().unwrap());
        rules.insert(13, "14 3 | 1 12".parse::<Rule>().unwrap());
        rules.insert(15, "1 | 14".parse::<Rule>().unwrap());
        rules.insert(17, "14 2 | 1 7".parse::<Rule>().unwrap());
        rules.insert(23, "25 1 | 22 14".parse::<Rule>().unwrap());
        rules.insert(28, "16 1".parse::<Rule>().unwrap());
        rules.insert(4, "1 1".parse::<Rule>().unwrap());
        rules.insert(20, "14 14 | 1 15".parse::<Rule>().unwrap());
        rules.insert(3, "5 14 | 16 1".parse::<Rule>().unwrap());
        rules.insert(27, "1 6 | 14 18".parse::<Rule>().unwrap());
        rules.insert(14, "\"b\"".parse::<Rule>().unwrap());
        rules.insert(21, "14 1 | 1 14".parse::<Rule>().unwrap());
        rules.insert(25, "1 1 | 1 14".parse::<Rule>().unwrap());
        rules.insert(22, "14 14".parse::<Rule>().unwrap());
        rules.insert(8, "42 | 42 8".parse::<Rule>().unwrap());
        rules.insert(26, "14 22 | 1 20".parse::<Rule>().unwrap());
        rules.insert(18, "15 15".parse::<Rule>().unwrap());
        rules.insert(7, "14 5 | 1 21".parse::<Rule>().unwrap());
        rules.insert(24, "14 1".parse::<Rule>().unwrap());

        assert!(!is_valid(&rules, "abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa", 0));
        assert!(is_valid(&rules, "bbabbbbaabaabba", 0));
        assert!(is_valid(&rules, "babbbbaabbbbbabbbbbbaabaaabaaa", 0));
        assert!(is_valid(&rules, "aaabbbbbbaaaabaababaabababbabaaabbababababaaa", 0));
        assert!(is_valid(&rules, "bbbbbbbaaaabbbbaaabbabaaa", 0));
        assert!(is_valid(&rules, "bbbababbbbaaaaaaaabbababaaababaabab", 0));
        assert!(is_valid(&rules, "ababaaaaaabaaab", 0));
        assert!(is_valid(&rules, "ababaaaaabbbaba", 0));
        assert!(is_valid(&rules, "baabbaaaabbaaaababbaababb", 0));
        assert!(is_valid(&rules, "abbbbabbbbaaaababbbbbbaaaababb", 0));
        assert!(is_valid(&rules, "aaaaabbaabaaaaababaa", 0));
        assert!(!is_valid(&rules, "aaaabbaaaabbaaa", 0));
        assert!(is_valid(&rules, "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa", 0));
        assert!(!is_valid(&rules, "babaaabbbaaabaababbaabababaaab", 0));
        assert!(is_valid(&rules, "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba", 0));
    }

    fn is_valid(rules: &HashMap<usize, Rule>, line: &str, rule: usize) -> bool {
        match rules.get(&rule) {
            Some(r) => r.test(rules, line),
            None => false,
        }
    }
}
