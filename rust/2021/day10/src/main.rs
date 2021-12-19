extern crate rdcl_aoc_helpers;

use std::fs::File;
use std::str::FromStr;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::input::WithReadLines;
use rdcl_aoc_helpers::parse_error;

fn main() {
    let args = get_args(&["<input file>"], 1);

    let results = File::open(&args[1])
        .read_lines(1)
        .collect::<Vec<Analysis>>();

    let invalid_score = results
        .iter()
        .filter(|r| matches!(r, Analysis::Invalid(_)))
        .map(|r| r.score())
        .sum::<u64>();

    println!("The total score of all invalid lines is {}.", invalid_score);

    let mut valid_scores = results
        .iter()
        .filter(|r| matches!(r, Analysis::Valid(_)))
        .map(|r| r.score())
        .collect::<Vec<u64>>();
    valid_scores.sort_unstable();

    println!(
        "The middle score of all valid lines is {}.",
        valid_scores[valid_scores.len() / 2],
    );
}

#[derive(Debug, Eq, PartialEq)]
enum Analysis {
    Invalid(char),
    Valid(String),
}

impl Analysis {
    fn score(&self) -> u64 {
        match self {
            Analysis::Invalid(')') => 3,
            Analysis::Invalid(']') => 57,
            Analysis::Invalid('}') => 1197,
            Analysis::Invalid('>') => 25137,
            Analysis::Invalid(ch) => panic!("illegal character encountered: {}", ch),
            Analysis::Valid(stack) => stack
                .chars()
                .map(|ch| match ch {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => panic!("illegal character encountered: {}", ch),
                })
                .fold(0u64, |acc, v| 5 * acc + v),
        }
    }
}

impl FromStr for Analysis {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stack = vec![];
        for ch in s.chars() {
            match ch {
                '(' | '[' | '{' | '<' => stack.push(ch),
                ')' | ']' | '}' | '>' => match stack.pop() {
                    Some('(') if ch == ')' => {}
                    Some('[') if ch == ']' => {}
                    Some('{') if ch == '}' => {}
                    Some('<') if ch == '>' => {}
                    _ => return Ok(Analysis::Invalid(ch)),
                },
                _ => return Err(parse_error!("invalid character encountered: {}", ch)),
            }
        }
        let missing = stack
            .iter()
            .rev()
            .map(|&ch| match ch {
                '(' => ')',
                '[' => ']',
                '{' => '}',
                '<' => '>',
                _ => panic!("illegal state encountered (stack={:?})", stack),
            })
            .collect::<String>();
        Ok(Analysis::Valid(missing))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = "[({(<(())[]>[[{[]{<()<>>".parse::<Analysis>().unwrap();
        assert_eq!(result, Analysis::Valid("}}]])})]".to_string()));
        assert_eq!(result.score(), 288957);
    }

    #[test]
    fn test_2() {
        let result = "[(()[<>])]({[<{<<[]>>(".parse::<Analysis>().unwrap();
        assert_eq!(result, Analysis::Valid(")}>]})".to_string()));
        assert_eq!(result.score(), 5566);
    }

    #[test]
    fn test_3() {
        let result = "{([(<{}[<>[]}>{[]{[(<()>".parse::<Analysis>().unwrap();
        assert_eq!(result, Analysis::Invalid('}'));
        assert_eq!(result.score(), 1197);
    }

    #[test]
    fn test_4() {
        let result = "(((({<>}<{<{<>}{[]{[]{}".parse::<Analysis>().unwrap();
        assert_eq!(result, Analysis::Valid("}}>}>))))".to_string()));
        assert_eq!(result.score(), 1480781);
    }

    #[test]
    fn test_5() {
        let result = "[[<[([]))<([[{}[[()]]]".parse::<Analysis>().unwrap();
        assert_eq!(result, Analysis::Invalid(')'));
        assert_eq!(result.score(), 3);
    }

    #[test]
    fn test_6() {
        let result = "[{[{({}]{}}([{[{{{}}([]".parse::<Analysis>().unwrap();
        assert_eq!(result, Analysis::Invalid(']'));
        assert_eq!(result.score(), 57);
    }

    #[test]
    fn test_7() {
        let result = "{<[[]]>}<{[{[{[]{()[[[]".parse::<Analysis>().unwrap();
        assert_eq!(result, Analysis::Valid("]]}}]}]}>".to_string()));
        assert_eq!(result.score(), 995444);
    }

    #[test]
    fn test_8() {
        let result = "[<(<(<(<{}))><([]([]()".parse::<Analysis>().unwrap();
        assert_eq!(result, Analysis::Invalid(')'));
        assert_eq!(result.score(), 3);
    }

    #[test]
    fn test_9() {
        let result = "<{([([[(<>()){}]>(<<{{".parse::<Analysis>().unwrap();
        assert_eq!(result, Analysis::Invalid('>'));
        assert_eq!(result.score(), 25137);
    }

    #[test]
    fn test_10() {
        let result = "<{([{{}}[<[[[<>{}]]]>[]]".parse::<Analysis>().unwrap();
        assert_eq!(result, Analysis::Valid("])}>".to_string()));
        assert_eq!(result.score(), 294);
    }
}
