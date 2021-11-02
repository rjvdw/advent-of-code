use std::fmt;
use std::str::FromStr;

use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::parse_error;

#[derive(Debug, Copy, Clone)]
pub struct Claim {
    nr: usize,
    at: (u64, u64),
    size: (u64, u64),
}

impl Claim {
    pub fn get_nr(&self) -> usize {
        self.nr
    }
}

impl IntoIterator for &Claim {
    type Item = (u64, u64);
    type IntoIter = ClaimIter;

    fn into_iter(self) -> Self::IntoIter {
        ClaimIter {
            lower: self.at,
            upper: (self.at.0 + self.size.0, self.at.1 + self.size.1),
            next_item: Some(self.at),
        }
    }
}

pub struct ClaimIter {
    lower: (u64, u64),
    upper: (u64, u64),
    next_item: Option<(u64, u64)>,
}

impl Iterator for ClaimIter {
    type Item = (u64, u64);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((x, y)) = self.next_item {
            if y + 1 < self.upper.1 {
                self.next_item = Some((x, y + 1));
            } else if x + 1 < self.upper.0 {
                self.next_item = Some((x + 1, self.lower.1));
            } else {
                self.next_item = None;
            }

            Some((x, y))
        } else {
            None
        }
    }
}

impl fmt::Display for Claim {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "#{} @ {},{}: {}x{}",
            self.nr, self.at.0, self.at.1, self.size.0, self.size.1
        )
    }
}

impl FromStr for Claim {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err = || parse_error!("Invalid input: {}", s);
        let nr_start_idx = 1;
        let nr_end_idx = nr_start_idx + s[nr_start_idx..].find(' ').ok_or_else(err)?;
        let at_x_start_idx = nr_end_idx + 3 + s[nr_end_idx..].find(" @ ").ok_or_else(err)?;
        let at_x_end_idx = at_x_start_idx + s[at_x_start_idx..].find(',').ok_or_else(err)?;
        let at_y_start_idx = at_x_end_idx + 1;
        let at_y_end_idx = at_y_start_idx + s[at_y_start_idx..].find(": ").ok_or_else(err)?;
        let size_x_start_idx = at_y_end_idx + 2;
        let size_x_end_idx = size_x_start_idx + s[size_x_start_idx..].find('x').ok_or_else(err)?;
        let size_y_start_idx = size_x_end_idx + 1;

        Ok(Claim {
            nr: s[nr_start_idx..nr_end_idx].parse()?,
            at: (
                s[at_x_start_idx..at_x_end_idx].parse()?,
                s[at_y_start_idx..at_y_end_idx].parse()?,
            ),
            size: (
                s[size_x_start_idx..size_x_end_idx].parse()?,
                s[size_y_start_idx..].parse()?,
            ),
        })
    }
}
