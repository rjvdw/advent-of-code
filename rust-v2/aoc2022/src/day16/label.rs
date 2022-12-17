use core::fmt;
use std::str::FromStr;

use rdcl_aoc_core::err_parse_error;
use rdcl_aoc_core::error::ParseError;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Label(pub char, pub char);

impl fmt::Debug for Label {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

impl FromStr for Label {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        match chars.next() {
            Some(ch1) => match chars.next() {
                Some(ch2) => Ok(Label(ch1, ch2)),
                None => err_parse_error!("Invalid valve: {}", s),
            },
            None => err_parse_error!("Invalid valve: {}", s),
        }
    }
}
