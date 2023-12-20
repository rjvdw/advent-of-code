use std::fmt;
use std::str::FromStr;

use rdcl_aoc2023::str_encoder::{decode_str, encode_str};
use rdcl_aoc_core::error::ParseError;

use crate::workflow::Label;

#[derive(Debug, Clone, Default)]
pub struct Part {
    ratings: Vec<(u32, u32)>,
}

impl Part {
    pub fn value(&self, label: Label) -> Option<u32> {
        match label {
            Label::Workflow(label) => self
                .ratings
                .iter()
                .find(|rating| rating.0 == label)
                .map(|rating| rating.1),
            Label::Accepted => None,
            Label::Rejected => None,
        }
    }

    pub fn score(&self) -> u32 {
        self.ratings.iter().map(|rating| rating.1).sum()
    }
}

impl FromStr for Part {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut part = Part::default();

        for rating in s[1..s.len() - 1].split(',') {
            let idx = rating.find('=').ok_or(())?;
            let label = encode_str(&rating[..idx]);
            let value = rating[idx + 1..].parse()?;

            part.ratings.push((label, value));
        }

        Ok(part)
    }
}

impl fmt::Display for Part {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        let mut first = true;
        for &(label, value) in &self.ratings {
            if !first {
                write!(f, ",")?;
            }
            let label = decode_str(label);
            write!(f, "{label}={value}")?;
            first = false;
        }
        write!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        let p1 = part("{x=787,m=2655,a=1222,s=2876}");
        assert_eq!(
            p1.ratings,
            vec![
                (b'x' as u32, 787),
                (b'm' as u32, 2655),
                (b'a' as u32, 1222),
                (b's' as u32, 2876),
            ]
        );
    }

    #[test]
    fn test_to_string() {
        let p1 = part("{x=787,m=2655,a=1222,s=2876}");
        assert_eq!(format!("{p1}"), "{x=787,m=2655,a=1222,s=2876}");
    }

    fn part(s: &str) -> Part {
        Part::from_str(s).unwrap()
    }
}
