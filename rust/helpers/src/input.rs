//! I/O operations.
use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Read};
use std::str::FromStr;
use std::{fmt, io, marker, mem};

use crate::error::WithOrExit;

/// Iterator over the lines that have been mapped to the requested type.
pub struct MappedLines<T: FromStr, R: Read> {
    exit_code_on_fail: i32,
    lines: Lines<BufReader<R>>,
    _marker: marker::PhantomData<T>,
}

impl<T: FromStr, R: Read> Iterator for MappedLines<T, R>
where
    <T as FromStr>::Err: fmt::Debug,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.lines
            .next()
            .map(|line| line.or_exit_with(self.exit_code_on_fail))
            .map(|line| line.parse::<T>())
            .map(|record| record.or_exit_with(self.exit_code_on_fail))
    }
}

/// This trait allows you to easily read lines from a Readable object.
pub trait WithReadLines<R: Read> {
    /// Read lines from a given Readable object, and parse each line to the requested type.
    fn read_lines<T: FromStr>(self, exit_code_on_fail: i32) -> MappedLines<T, R>;
}

impl WithReadLines<File> for File {
    fn read_lines<T: FromStr>(self, exit_code_on_fail: i32) -> MappedLines<T, File> {
        MappedLines {
            exit_code_on_fail,
            lines: BufReader::new(self).lines(),
            _marker: Default::default(),
        }
    }
}

impl WithReadLines<File> for io::Result<File> {
    fn read_lines<T: FromStr>(self, exit_code_on_fail: i32) -> MappedLines<T, File> {
        self.or_exit_with(exit_code_on_fail)
            .read_lines(exit_code_on_fail)
    }
}

/// This trait allows you to easily convert an object to a vec of items of the required type.
pub trait WithAsRecords {
    /// Convert to a list of records of the requested type.
    fn as_records<T: FromStr>(&self) -> Result<Vec<T>, <T as FromStr>::Err>;
}

impl<'a> WithAsRecords for Vec<&'a str> {
    fn as_records<T: FromStr>(&self) -> Result<Vec<T>, <T as FromStr>::Err> {
        let mut records = Vec::new();
        for line in self {
            records.push(line.parse::<T>()?);
        }
        Ok(records)
    }
}

/// Inspired by `std::str::FromStr`, so you can read input files where a record spans multiple
/// lines.
pub trait MultilineFromStr {
    /// The associated error which can be returned from parsing.
    type Err;

    /// Create a new initial record.
    fn new() -> Self;

    /// Test for whether a line indicates that a new record starts.
    fn indicates_new_record(&self, line: &str) -> bool;

    /// Parses a line.
    fn parse(&mut self, line: &str) -> Result<(), Self::Err>;
}

/// Iterator over the lines that have been mapped to the requested type.
pub struct MappedMultiLines<T, U>
where
    T: MultilineFromStr,
    U: Iterator<Item = io::Result<String>>,
{
    exit_code_on_fail: i32,
    lines: U,
    current: T,
    exhausted: bool,
}

impl<T, U> Iterator for MappedMultiLines<T, U>
where
    T: MultilineFromStr,
    U: Iterator<Item = io::Result<String>>,
    <T as MultilineFromStr>::Err: fmt::Debug,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.exhausted {
            None
        } else {
            let mut record = mem::replace(&mut self.current, T::new());
            self.exhausted = true;
            for line in &mut self.lines {
                let line = line.or_exit_with(self.exit_code_on_fail);
                if self.current.indicates_new_record(&line) {
                    self.exhausted = false;
                    self.current
                        .parse(&line)
                        .or_exit_with(self.exit_code_on_fail);
                    break;
                }
                record.parse(&line).or_exit_with(self.exit_code_on_fail);
            }

            Some(record)
        }
    }
}

/// This trait allows you to easily read lines from a Readable object.
pub trait WithReadMultiLines<R: Read> {
    /// Read lines from a given Readable object, and parse its lines to the requested type.
    fn read_multi_lines<T: MultilineFromStr>(
        self,
        exit_code_on_fail: i32,
    ) -> MappedMultiLines<T, Lines<BufReader<R>>>;
}

impl WithReadMultiLines<File> for File {
    fn read_multi_lines<T: MultilineFromStr>(
        self,
        exit_code_on_fail: i32,
    ) -> MappedMultiLines<T, Lines<BufReader<File>>> {
        MappedMultiLines {
            exit_code_on_fail,
            lines: BufReader::new(self).lines(),
            current: T::new(),
            exhausted: false,
        }
    }
}

impl WithReadMultiLines<File> for io::Result<File> {
    fn read_multi_lines<T: MultilineFromStr>(
        self,
        exit_code_on_fail: i32,
    ) -> MappedMultiLines<T, Lines<BufReader<File>>> {
        self.or_exit_with(exit_code_on_fail)
            .read_multi_lines(exit_code_on_fail)
    }
}

/// This trait allows you to easily convert an object to a vec of items of the required type.
pub trait WithAsMultilineRecords {
    /// Convert to a list of records of the requested type.
    fn as_multiline_records<T: MultilineFromStr>(
        &self,
    ) -> Result<Vec<T>, <T as MultilineFromStr>::Err>;
}

impl<'a> WithAsMultilineRecords for Vec<&'a str> {
    fn as_multiline_records<T: MultilineFromStr>(
        &self,
    ) -> Result<Vec<T>, <T as MultilineFromStr>::Err> {
        let mut records = Vec::new();
        let mut record = T::new();
        for line in self {
            if record.indicates_new_record(line) {
                records.push(record);
                record = T::new();
            }
            record.parse(line)?;
        }
        records.push(record);
        Ok(records)
    }
}

#[cfg(test)]
mod tests {
    use crate::error::ParseError;

    use super::*;

    #[test]
    fn test_simple() {
        let input = vec!["1", "2", "3", "4", "5"].as_records::<u8>().unwrap();

        assert_eq!(input, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_multiline() {
        let input = vec!["1", "2", "", "3", "4", "", "5"]
            .as_multiline_records::<Record>()
            .unwrap();

        assert_eq!(
            input,
            vec![
                Record { items: vec![1, 2] },
                Record { items: vec![3, 4] },
                Record { items: vec![5] },
            ]
        );
    }

    #[derive(Debug, Eq, PartialEq)]
    pub struct Record {
        items: Vec<u8>,
    }

    impl MultilineFromStr for Record {
        type Err = ParseError;

        fn new() -> Self {
            Record { items: Vec::new() }
        }

        fn indicates_new_record(&self, line: &str) -> bool {
            line.is_empty()
        }

        fn parse(&mut self, line: &str) -> Result<(), Self::Err> {
            if !line.is_empty() {
                self.items.push(line.parse::<u8>()?);
            }

            Ok(())
        }
    }
}
