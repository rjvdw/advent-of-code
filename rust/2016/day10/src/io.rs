use std::str::FromStr;

use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::parse_error;

#[derive(Debug, Clone)]
pub enum Input {
    Bin(usize, Output),
    Bot(usize, Output, Output),
}

impl FromStr for Input {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let error = || parse_error!("Invalid input: {}", s);

        if let Some(r) = s.strip_prefix("bot ") {
            let (idx, nr) = parse_up_to_space(r)?;
            let r = &r[idx + " gives low to ".len()..];
            if let Some(idx) = r.find(" and high to ") {
                let output1 = r[..idx].parse::<Output>()?;
                let output2 = r[idx + " and high to ".len()..].parse::<Output>()?;

                Ok(Input::Bot(nr, output1, output2))
            } else {
                Err(error())
            }
        } else if let Some(r) = s.strip_prefix("value ") {
            let (idx, nr) = parse_up_to_space(r)?;
            let output = r[idx + " goes to ".len()..].parse()?;
            Ok(Input::Bin(nr, output))
        } else {
            Err(error())
        }
    }
}

fn parse_up_to_space(r: &str) -> Result<(usize, usize), ParseError> {
    if let Some(idx) = r.find(' ') {
        Ok((idx, r[..idx].parse()?))
    } else {
        Err(parse_error!("Invalid input"))
    }
}

#[derive(Debug, Clone)]
pub enum Output {
    Bin(usize),
    Bot(usize),
}

impl FromStr for Output {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(r) = s.strip_prefix("bot ") {
            Ok(Output::Bot(r.parse()?))
        } else if let Some(r) = s.strip_prefix("output ") {
            Ok(Output::Bin(r.parse()?))
        } else {
            Err(parse_error!("Invalid output: {}", s))
        }
    }
}
