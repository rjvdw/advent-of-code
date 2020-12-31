use std::fs::File;
use std::io::{BufRead, BufReader};

use rdcl_aoc_helpers::error::ParseError;

use crate::item::Item;

pub fn parse_input(path: &str) -> Result<[Vec<Item>; 4], ParseError> {
    let mut floors = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];
    let file = File::open(path)?;
    for line in BufReader::new(file).lines() {
        let (floor, idx) = parse_line(&line?)?;
        floors[idx] = floor;
    }
    Ok(floors)
}

pub fn parse_line(line: &str) -> Result<(Vec<Item>, usize), ParseError> {
    let (idx, r) = if let Some(r) = line.strip_prefix("The first floor contains ") {
        (0, r)
    } else if let Some(r) = line.strip_prefix("The second floor contains ") {
        (1, r)
    } else if let Some(r) = line.strip_prefix("The third floor contains ") {
        (2, r)
    } else if let Some(r) = line.strip_prefix("The fourth floor contains ") {
        (3, r)
    } else {
        return Err(ParseError(format!("Invalid input: {}", line)));
    };

    let mut floor = Vec::new();

    if r != "nothing relevant." {
        for part in r.split(", ").flat_map(|p| p.split(" and ")) {
            let item = part
                .trim()
                .trim_start_matches("and a ")
                .trim_start_matches("a ")
                .trim_end_matches('.')
                .parse::<Item>()?;

            floor.push(item);
        }
    }

    Ok((floor, idx))
}
