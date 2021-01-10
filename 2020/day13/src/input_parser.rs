use std::fs::File;
use std::io::{BufRead, BufReader};

use rdcl_aoc_helpers::err_parse_error;
use rdcl_aoc_helpers::error::ParseError;

pub fn read(path: &str) -> Result<(u32, Vec<u32>), ParseError> {
    let file = File::open(path)?;
    let mut lines = BufReader::new(file).lines();
    let earliest_departure = match lines.next() {
        Some(Ok(line)) => Ok(line.parse::<u32>()?),
        _ => err_parse_error!("Input file has insufficient lines"),
    }?;
    let mut schedule = Vec::new();
    match lines.next() {
        Some(Ok(line)) => {
            for item in line.split(',') {
                if item == "x" {
                    schedule.push(0);
                } else {
                    schedule.push(item.parse::<u32>()?);
                }
            }
            Ok(())
        }
        _ => err_parse_error!("Input file has insufficient lines"),
    }?;

    Ok((earliest_departure, schedule))
}
