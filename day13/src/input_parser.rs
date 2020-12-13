use std::fs::File;
use std::io::{BufRead, BufReader};

use helpers::ParseError;

pub fn read(path: &str) -> Result<(u32, Vec<u32>), ParseError> {
    let file = File::open(path)?;
    let mut lines = BufReader::new(file).lines();
    let earliest_departure = match lines.next() {
        Some(Ok(line)) => Ok(line.parse::<u32>()?),
        _ => Err(ParseError("Input file has insufficient lines".to_string())),
    }?;
    let mut schedule = Vec::new();
    match lines.next() {
        Some(Ok(line)) => {
            for line in line.split(',') {
                if line == "x" {
                    schedule.push(0);
                } else {
                    schedule.push(line.parse::<u32>()?);
                }
            }
            Ok(())
        }
        _ => Err(ParseError("Input file has insufficient lines".to_string())),
    }?;

    Ok((earliest_departure, schedule))
}
