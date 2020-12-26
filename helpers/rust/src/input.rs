use std::io::{BufRead, BufReader, Lines, Read};

use crate::error::WithOrExit;
use std::fs::File;
use std::str::FromStr;
use std::{fmt, io, marker, mem};

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
pub struct MappedMultiLines<T: MultilineFromStr, R: Read> {
    exit_code_on_fail: i32,
    lines: Lines<BufReader<R>>,
    current: T,
    exhausted: bool,
}

impl<T: MultilineFromStr, R: Read> Iterator for MappedMultiLines<T, R>
where
    <T as MultilineFromStr>::Err: fmt::Debug,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.exhausted {
            None
        } else {
            let mut record = mem::replace(&mut self.current, T::new());
            self.exhausted = true;
            while let Some(line) = self.lines.next() {
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

pub trait WithReadMultiLines<R: Read> {
    /// Read lines from a given Readable object, and parse each line to the requested type.
    fn read_multi_lines<T: MultilineFromStr>(
        self,
        exit_code_on_fail: i32,
    ) -> MappedMultiLines<T, R>;
}

impl WithReadMultiLines<File> for File {
    fn read_multi_lines<T: MultilineFromStr>(
        self,
        exit_code_on_fail: i32,
    ) -> MappedMultiLines<T, File> {
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
    ) -> MappedMultiLines<T, File> {
        self.or_exit_with(exit_code_on_fail)
            .read_multi_lines(exit_code_on_fail)
    }
}

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
