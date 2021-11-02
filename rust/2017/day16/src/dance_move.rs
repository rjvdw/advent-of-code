use std::collections::VecDeque;
use std::fmt;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::parse_error;

#[derive(Debug, Copy, Clone)]
pub enum DanceMove {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

impl DanceMove {
    pub fn execute(&self, programs: &mut VecDeque<char>) {
        match self {
            DanceMove::Spin(spin) => {
                programs.rotate_right(*spin % programs.len());
            }
            DanceMove::Exchange(left, right) => {
                programs.swap(*left, *right);
            }
            DanceMove::Partner(left, right) => {
                let left = programs.iter().position(|c| c == left);
                let right = programs.iter().position(|c| c == right);

                if let (Some(left), Some(right)) = (left, right) {
                    programs.swap(left, right);
                }
            }
        }
    }
}

impl fmt::Display for DanceMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DanceMove::Spin(spin) => write!(f, "s{}", spin),
            DanceMove::Exchange(left, right) => write!(f, "x{}/{}", left, right),
            DanceMove::Partner(left, right) => write!(f, "p{}/{}", left, right),
        }
    }
}

impl FromStr for DanceMove {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(r) = s.strip_prefix('s') {
            Ok(DanceMove::Spin(r.parse()?))
        } else if let Some(r) = s.strip_prefix('x') {
            let (left, right) = parse_pair(r)?;
            Ok(DanceMove::Exchange(left, right))
        } else if let Some(r) = s.strip_prefix('p') {
            let (left, right) = parse_pair(r)?;
            Ok(DanceMove::Partner(left, right))
        } else {
            Err(parse_error!("Invalid dance move: {}", s))
        }
    }
}

pub fn parse_input<R: Read>(readable: R) -> Result<Vec<DanceMove>, ParseError> {
    let mut dance_moves = vec![];
    for line in BufReader::new(readable).lines() {
        let line = line?;
        for dance_move in line.split(',') {
            dance_moves.push(dance_move.parse()?);
        }
    }
    Ok(dance_moves)
}

fn parse_pair<T: FromStr>(s: &str) -> Result<(T, T), ParseError>
where
    ParseError: From<<T as FromStr>::Err>,
{
    if let Some(idx) = s.find('/') {
        Ok((s[..idx].parse()?, s[idx + 1..].parse()?))
    } else {
        Err(parse_error!("Invalid pair: {}", s))
    }
}
