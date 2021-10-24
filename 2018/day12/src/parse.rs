use std::fs::File;
use std::io::{BufRead, BufReader};

use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::parse_error;

pub const RULE_SIZE: usize = 5;

pub fn parse_input(path: &str) -> Result<(Vec<bool>, u32), ParseError> {
    let file = File::open(path)?;
    let mut initial_state = Vec::new();
    let mut rules = 0;

    for line in BufReader::new(file).lines() {
        let line = line?;
        match parse_line(&line)? {
            ParseResult::EmptyLine => {}
            ParseResult::InitialState(state) => {
                initial_state = state;
            }
            ParseResult::Rule(rule, active) => {
                if active {
                    if rule == 0 {
                        return Err(parse_error!("Rule '{}' is illegal.", line));
                    }
                    rules += 2u32.pow(rule);
                }
            }
        }
    }

    Ok((initial_state, rules))
}

#[cfg(test)]
pub fn parse_test_input(lines: &[&str]) -> (Vec<bool>, u32) {
    let mut initial_state = Vec::new();
    let mut rules = 0;

    for line in lines {
        match parse_line(line).unwrap() {
            ParseResult::EmptyLine => {}
            ParseResult::InitialState(state) => {
                initial_state = state;
            }
            ParseResult::Rule(rule, active) => {
                if active {
                    if rule == 0 {
                        panic!("Rule '{}' is illegal.", line);
                    }
                    rules += 2u32.pow(rule);
                }
            }
        }
    }

    (initial_state, rules)
}

enum ParseResult {
    EmptyLine,
    InitialState(Vec<bool>),
    Rule(u32, bool),
}

fn parse_line(line: &str) -> Result<ParseResult, ParseError> {
    if line.is_empty() {
        Ok(ParseResult::EmptyLine)
    } else if let Some(line) = line.strip_prefix("initial state: ") {
        let mut initial_state = vec![false; line.len()];
        for (idx, ch) in line.chars().enumerate() {
            if ch == '#' {
                initial_state[idx] = true;
            }
        }
        Ok(ParseResult::InitialState(initial_state))
    } else {
        let mut rule = 0;
        for ch in line.chars().take(RULE_SIZE) {
            rule <<= 1;
            if ch == '#' {
                rule += 1;
            }
        }
        Ok(ParseResult::Rule(rule, line.ends_with('#')))
    }
}
