use std::fs::File;
use std::io::{BufRead, BufReader};

use helpers::parse_error::ParseError;

pub fn read(path: &str) -> Result<Vec<(i32, i32)>, ParseError> {
    let file = File::open(path)?;
    let mut input = Vec::new();

    for (y, line) in BufReader::new(file).lines().enumerate() {
        let line = line?;
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                input.push((x as i32, y as i32));
            }
        }
    }

    Ok(input)
}
