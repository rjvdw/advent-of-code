use std::str::FromStr;

use rdcl_aoc_helpers::error::ParseError;

#[derive(Debug, Copy, Clone)]
pub struct Point {
    position: (i64, i64),
    velocity: (i64, i64),
}

impl Point {
    pub fn get_position(&self) -> (i64, i64) {
        self.position
    }

    pub fn tick(&mut self) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
    }

    pub fn rewind(&mut self) {
        self.position.0 -= self.velocity.0;
        self.position.1 -= self.velocity.1;
    }
}

impl FromStr for Point {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (offset, position) = parse_pair(s, 0)?;
        let (_, velocity) = parse_pair(s, offset)?;

        Ok(Point { position, velocity })
    }
}

fn parse_pair(s: &str, offset: usize) -> Result<(usize, (i64, i64)), ParseError> {
    let opts = (
        s[offset..].find('<'),
        s[offset..].find(','),
        s[offset..].find('>'),
    );

    if let (Some(i_start), Some(i_mid), Some(i_end)) = opts {
        let left = s[offset + i_start + 1..offset + i_mid].trim().parse()?;
        let right = s[offset + i_mid + 1..offset + i_end].trim().parse()?;

        Ok((offset + i_end + 1, (left, right)))
    } else {
        Err(ParseError(format!("Invalid input line: {}", s)))
    }
}
