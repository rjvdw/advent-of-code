use std::fmt;
use std::str::FromStr;

use rdcl_aoc_core::err_parse_error;
use rdcl_aoc_core::error::ParseError;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Slope(pub usize, pub usize);

impl fmt::Display for Slope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.0, self.1)
    }
}

impl FromStr for Slope {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.find('/') {
            Some(pos) => {
                let right = s[..pos].parse()?;
                let down = s[pos + 1..].parse()?;
                Ok(Slope(right, down))
            }
            None => {
                err_parse_error!("Invalid argument: {}", s)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!("1/3".parse::<Slope>().unwrap(), Slope(1, 3));
        assert!("3/".parse::<Slope>().is_err());
        assert!("asd".parse::<Slope>().is_err());
    }
}
