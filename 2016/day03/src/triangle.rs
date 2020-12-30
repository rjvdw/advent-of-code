use std::str::FromStr;

use rdcl_aoc_helpers::error::ParseError;

pub struct Triangle(pub u32, pub u32, pub u32);

impl Triangle {
    pub fn is_possible(&self) -> bool {
        self.0 + self.1 > self.2 && self.1 + self.2 > self.0 && self.0 + self.2 > self.1
    }
}

impl FromStr for Triangle {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sides = Vec::new();
        for side in s.split_whitespace() {
            sides.push(side.parse()?);
        }

        if sides.len() != 3 {
            Err(ParseError(format!("Invalid input: {}", s)))
        } else {
            Ok(Triangle(sides[0], sides[1], sides[2]))
        }
    }
}
