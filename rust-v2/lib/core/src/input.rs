//! Helper to read Advent of Code input files.
//!
//! # Usage
//!
//! ```
//! use rdcl_aoc_core::input::InputReader;
//!
//! use std::str::FromStr;
//! use std::path::PathBuf;
//!
//! fn example() {
//!     let input = InputReader::from("path/to/input/file");
//!     let lines = input.read_lines().collect::<Vec<String>>();
//!     let parsed = input.parse_lines(u32::from_str).collect::<Vec<u32>>();
//! }
//! ```

use core::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::exit;

use crate::error::ParseError;

/// Contains the location of the input file and allows for operations to be performed on said file.
pub struct InputReader<P: AsRef<Path>> {
    input: P,
}

impl<P: AsRef<Path>> InputReader<P> {
    /// Construct a new input reader from a given input path
    pub fn from(input: P) -> InputReader<P> {
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

    /// Reads a single line from the file. Useful if the input is a single line.
    pub fn read_line(&self) -> String {
        match BufReader::new(self.open_file()).lines().next() {
            Some(Ok(line)) => line,
            Some(Err(e)) => {
                eprintln!("Failed to read line: {}", e);
                exit(1);
            }
            None => {
                eprintln!("Input file is empty");
                exit(1);
            }
        }
    }

    /// Reads the lines from the file and transforms them with the provided parser
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

    /// Reads and parses all lines in the file
    pub fn parse<T: FromInput>(&self) -> T {
        match T::parse(self.read_lines()) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Failed to parse input: {:?}", e);
                exit(1);
            }
        }
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

/// Types that can be constructed by reading the entire input.
pub trait FromInput: Sized {
    fn parse<T>(input: T) -> Result<Self, ParseError>
    where
        T: Iterator<Item = String>;
}
