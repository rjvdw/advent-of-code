use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;
use std::str::FromStr;

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
    for line in BufReader::new(file).lines() {
        let line = line.expect("Failed to read line");
        if T::indicates_new_record(&line) {
            values.push(record);
            record = T::new();
        }
        record.parse(&line)?;
    }
    values.push(record);

    Ok(values)
}

/// Mirrors `std::str::FromStr`, but slightly modified so it can be used to parse record that span
/// multiple lines.
pub trait FromMultilineStr {
    /// The associated error which can be returned from parsing.
    type Err;

    /// Create a new initial record.
    fn new() -> Self;

    /// Test for whether a line indicates that a new record starts.
    fn indicates_new_record(line: &String) -> bool;

    /// Parses a line.
    fn parse(&mut self, line: &String) -> Result<(), Self::Err>;
}

/// Helper method for dealing with results. If a result is an Err, it will print an error message
/// and terminate the process with an exit code of 1.
pub fn handle_result<T, E: Display>(res: Result<T, E>) -> T {
    match res {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    }
}

pub fn parse_input<I: FromStr>(input_lines: Vec<&str>) -> Result<Vec<I>, <I as FromStr>::Err> {
    let mut values: Vec<I> = Vec::with_capacity(input_lines.len());
    for line in input_lines {
        values.push(line.parse::<I>()?);
    }
    Ok(values)
}

/// Helper method for parsing input using FromMultilineStr. This method is mostly useful for unit
/// tests.
pub fn parse_multiline_input<I: FromMultilineStr>(
    input_lines: Vec<&str>,
) -> Result<Vec<I>, <I as FromMultilineStr>::Err> {
    let mut values = Vec::new();
    let mut record = I::new();
    for line in input_lines {
        let line = &line.to_string();
        if I::indicates_new_record(line) {
            values.push(record);
            record = I::new();
        }
        record.parse(line)?;
    }
    values.push(record);

    Ok(values)
}
