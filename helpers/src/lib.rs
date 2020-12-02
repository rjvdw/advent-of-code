use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub fn read_input<T: FromStr>(path: &str) -> Result<Vec<T>, <T as FromStr>::Err> {
    let file = File::open(path).expect("Failed to open file");
    let mut values = Vec::new();
    for line in BufReader::new(file).lines() {
        let line = line.expect("Failed to read line");
        let line = line.parse::<T>()?;
        values.push(line);
    }

    return Ok(values);
}
