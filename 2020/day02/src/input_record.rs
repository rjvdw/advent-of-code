use std::str::FromStr;

use helpers::parse_error::ParseError;

#[derive(Debug)]
pub struct InputRecord {
    idx1: usize,
    idx2: usize,
    character: char,
    password: String,
}

impl InputRecord {
    pub fn valid_according_to_old_job(&self) -> bool {
        let count = self
            .password
            .chars()
            .filter(|&c| c == self.character)
            .count();

        count >= self.idx1 && count <= self.idx2
    }

    pub fn valid_according_to_corporate_policy(&self) -> bool {
        let c1 = self
            .password
            .chars()
            .nth(self.idx1 - 1)
            .filter(|&c| c == self.character);
        let c2 = self
            .password
            .chars()
            .nth(self.idx2 - 1)
            .filter(|&c| c == self.character);

        (c1.is_some() && c2.is_none()) || (c1.is_none() && c2.is_some())
    }
}

impl FromStr for InputRecord {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p1 = find_char(s, 0, '-')?;
        let p2 = p1 + find_char(s, p1, ' ')?;
        let p3 = p2 + find_char(s, p2, ':')?;

        let idx1 = s[..p1].parse::<usize>()?;
        let idx2 = s[p1 + 1..p2].parse::<usize>()?;
        let character = match s.chars().nth(p2 + 1) {
            Some(v) => Ok(v),
            None => Err(ParseError(format!("Invalid character in line {}", s))),
        }?;
        let password = s.chars().skip(p3 + 2).collect();

        Ok(InputRecord {
            idx1,
            idx2,
            character,
            password,
        })
    }
}

fn find_char(s: &str, offset: usize, ch: char) -> Result<usize, ParseError> {
    match s.chars().skip(offset).position(|c| c == ch) {
        Some(v) => Ok(v),
        None => Err(ParseError(format!(
            "Input file has invalid format in line {}",
            s,
        ))),
    }
}
