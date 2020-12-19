extern crate helpers;

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;

use helpers::ParseError;

/// https://adventofcode.com/2020/day/19
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <input file>", &args[0]);
        exit(1);
    }

    match read(&args[1]) {
        Ok(result) => println!("The result is: {}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn read(path: &str) -> Result<usize, ParseError> {
    let mut lines = BufReader::new(File::open(path)?).lines();

    // first collect the rules
    let mut rules: HashMap<usize, String> = HashMap::new();
    while let Some(line) = lines.next() {
        let line = line?;

        if line.is_empty() {
            break;
        }

        match line.find(':') {
            Some(idx) => {
                let key = line[..idx].parse::<usize>()?;
                let value = line[idx + 2..].to_string();

                rules.insert(key, value);
            }
            None => return Err(ParseError(format!("Invalid rule encountered: {}", line))),
        }
    }

    // then check which messages match
    let mut count = 0;
    for line in lines {
        if is_valid(&rules, &line?, 0) {
            count += 1;
        }
    }

    Ok(count)
}

fn is_valid(rules: &HashMap<usize, String>, line: &str, for_rule: usize) -> bool {
    check_rule(&rules, &line, for_rule, 0)
        .iter()
        .any(|&matched_len| matched_len == line.len())
}

fn check_rule(
    rules: &HashMap<usize, String>,
    line: &str,
    for_rule: usize,
    position: usize,
) -> Vec<usize> {
    rules
        .get(&for_rule)
        .map_or(Vec::new(), |rule| rule.split(" | ").collect())
        .iter()
        .flat_map(|rule| matches(rules, line, rule, position))
        .collect()
}

fn matches(rules: &HashMap<usize, String>, line: &str, rule: &str, position: usize) -> Vec<usize> {
    let mut results = Vec::new();

    if position < line.len() {
        if rule.starts_with('"') {
            // Assumption: Quoted rules only contain one character.
            let ch = &rule[1..=1];
            if line[position..].starts_with(ch) {
                results.push(position + 1)
            }
        } else if let Some(idx) = rule.find(' ') {
            let part = rule[0..idx].parse::<usize>().unwrap(); // TODO: Better error handling?
            let rest = &rule[idx + 1..];

            for matched_len in check_rule(rules, line, part, position) {
                let m = matches(rules, line, rest, matched_len);
                results.extend(m);
            }
        } else {
            let part = rule.parse::<usize>().unwrap(); // TODO: Better error handling?
            let m = check_rule(rules, line, part, position);
            results.extend(m);
        }
    }

    results
}

#[cfg(test)]
#[rustfmt::skip::macros(assert)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let mut rules = HashMap::new();
        rules.insert(0, "1 2".to_string());
        rules.insert(1, "\"a\"".to_string());
        rules.insert(2, "1 3 | 3 1".to_string());
        rules.insert(3, "\"b\"".to_string());

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
        rules.insert(0, "4 1 5".to_string());
        rules.insert(1, "2 3 | 3 2".to_string());
        rules.insert(2, "4 4 | 5 5".to_string());
        rules.insert(3, "4 5 | 5 4".to_string());
        rules.insert(4, "\"a\"".to_string());
        rules.insert(5, "\"b\"".to_string());

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
        rules.insert(42, "9 14 | 10 1".to_string());
        rules.insert(9, "14 27 | 1 26".to_string());
        rules.insert(10, "23 14 | 28 1".to_string());
        rules.insert(1, "\"a\"".to_string());
        rules.insert(11, "42 31".to_string());
        rules.insert(5, "1 14 | 15 1".to_string());
        rules.insert(19, "14 1 | 14 14".to_string());
        rules.insert(12, "24 14 | 19 1".to_string());
        rules.insert(16, "15 1 | 14 14".to_string());
        rules.insert(31, "14 17 | 1 13".to_string());
        rules.insert(6, "14 14 | 1 14".to_string());
        rules.insert(2, "1 24 | 14 4".to_string());
        rules.insert(0, "8 11".to_string());
        rules.insert(13, "14 3 | 1 12".to_string());
        rules.insert(15, "1 | 14".to_string());
        rules.insert(17, "14 2 | 1 7".to_string());
        rules.insert(23, "25 1 | 22 14".to_string());
        rules.insert(28, "16 1".to_string());
        rules.insert(4, "1 1".to_string());
        rules.insert(20, "14 14 | 1 15".to_string());
        rules.insert(3, "5 14 | 16 1".to_string());
        rules.insert(27, "1 6 | 14 18".to_string());
        rules.insert(14, "\"b\"".to_string());
        rules.insert(21, "14 1 | 1 14".to_string());
        rules.insert(25, "1 1 | 1 14".to_string());
        rules.insert(22, "14 14".to_string());
        rules.insert(8, "42".to_string());
        rules.insert(26, "14 22 | 1 20".to_string());
        rules.insert(18, "15 15".to_string());
        rules.insert(7, "14 5 | 1 21".to_string());
        rules.insert(24, "14 1".to_string());

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
        rules.insert(42, "9 14 | 10 1".to_string());
        rules.insert(9, "14 27 | 1 26".to_string());
        rules.insert(10, "23 14 | 28 1".to_string());
        rules.insert(1, "\"a\"".to_string());
        rules.insert(11, "42 31 | 42 11 31".to_string());
        rules.insert(5, "1 14 | 15 1".to_string());
        rules.insert(19, "14 1 | 14 14".to_string());
        rules.insert(12, "24 14 | 19 1".to_string());
        rules.insert(16, "15 1 | 14 14".to_string());
        rules.insert(31, "14 17 | 1 13".to_string());
        rules.insert(6, "14 14 | 1 14".to_string());
        rules.insert(2, "1 24 | 14 4".to_string());
        rules.insert(0, "8 11".to_string());
        rules.insert(13, "14 3 | 1 12".to_string());
        rules.insert(15, "1 | 14".to_string());
        rules.insert(17, "14 2 | 1 7".to_string());
        rules.insert(23, "25 1 | 22 14".to_string());
        rules.insert(28, "16 1".to_string());
        rules.insert(4, "1 1".to_string());
        rules.insert(20, "14 14 | 1 15".to_string());
        rules.insert(3, "5 14 | 16 1".to_string());
        rules.insert(27, "1 6 | 14 18".to_string());
        rules.insert(14, "\"b\"".to_string());
        rules.insert(21, "14 1 | 1 14".to_string());
        rules.insert(25, "1 1 | 1 14".to_string());
        rules.insert(22, "14 14".to_string());
        rules.insert(8, "42 | 42 8".to_string());
        rules.insert(26, "14 22 | 1 20".to_string());
        rules.insert(18, "15 15".to_string());
        rules.insert(7, "14 5 | 1 21".to_string());
        rules.insert(24, "14 1".to_string());

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
}
