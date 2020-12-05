use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;
use std::str::FromStr;

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

pub fn read_multiline_input<T: FromMultilineStr>(path: &str) -> Result<Vec<T>, <T as FromMultilineStr>::Err> {
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

pub trait FromMultilineStr {
    type Err;

    fn new() -> Self;

    fn indicates_new_record(line: &String) -> bool;

    fn parse(&mut self, line: &String) -> Result<(), Self::Err>;
}

pub fn handle_result<T, E: Display>(res: Result<T, E>) -> T {
    match res {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    }
}
