use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use crate::from_multiline_str::FromMultilineStr;

/// Reads input from a file, and parses each line using `std::str::FromStr`.
pub fn read_input<T: FromStr>(path: &str) -> Result<Vec<T>, <T as FromStr>::Err> {
    let file = File::open(path).expect("Failed to open file");
    let mut values = Vec::new();
    for line in BufReader::new(file).lines() {
        let line = line.expect("Failed to read line");
        let line = line.parse::<T>()?;
        values.push(line);
    }

    Ok(values)
}

/// Reads input from a file, and parses each line using `FromMultilineStr`.
pub fn read_multiline_input<T: FromMultilineStr>(
    path: &str,
) -> Result<Vec<T>, <T as FromMultilineStr>::Err> {
    let file = File::open(path).expect("Failed to open file");
    let mut values = Vec::new();
    let mut record = T::new();
    let mut discard = T::DISCARD_FIRST_RECORD;
    for line in BufReader::new(file).lines() {
        let line = line.expect("Failed to read line");
        if T::indicates_new_record(&line) {
            if !discard {
                values.push(record);
            }
            record = T::new();
            discard = false;
        }
        record.parse(&line)?;
    }
    if !discard {
        values.push(record);
    }

    Ok(values)
}

/// Reads input from a file, parses each line using `FromMultilineStr`, and returns a single record.
pub fn read_multiline_input_as_single<T: FromMultilineStr>(
    path: &str,
) -> Result<T, <T as FromMultilineStr>::Err> {
    let file = File::open(path).expect("Failed to open file");
    let mut record = T::new();
    for line in BufReader::new(file).lines() {
        let line = line.expect("Failed to read line");
        record.parse(&line)?;
    }

    Ok(record)
}
