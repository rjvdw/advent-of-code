use core::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::exit;

/// Helper to read Advent of Code input files
pub struct InputReader {
    input: PathBuf,
}

impl InputReader {
    /// Construct a new input reader from a given input path
    pub fn from(input: PathBuf) -> InputReader {
        InputReader { input }
    }

    /// Reads the lines from the file
    pub fn read_lines(&self) -> impl Iterator<Item = String> {
        BufReader::new(self.open_file()).lines().map(|s| match s {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to read line: {}", e);
                exit(1);
            }
        })
    }

    pub fn parse_lines<T, F, E>(&self, parser: F) -> impl Iterator<Item = T>
    where
        F: Fn(&str) -> Result<T, E> + 'static,
        E: fmt::Debug,
    {
        self.read_lines().map(move |line| match parser(&line) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Failed to parse line '{}': {:?}", line, e);
                exit(1);
            }
        })
    }

    /// Opens the file for reading
    fn open_file(&self) -> File {
        match File::open(&self.input) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Failed to open file: {}", e);
                exit(1);
            }
        }
    }
}
