use std::str::FromStr;

use helpers::parse_error::ParseError;

#[derive(Debug)]
pub struct InputRecord {
    line: Vec<bool>,
}

impl InputRecord {
    pub fn test_index(&self, i: usize) -> bool {
        self.line[i % self.line.len()]
    }
}

impl FromStr for InputRecord {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let line: Vec<bool> = s.chars().map(|x| x == '#').collect();
        Ok(InputRecord { line })
    }
}
