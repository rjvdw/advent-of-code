//! The solution for [advent of code 2020, day 4](https://adventofcode.com/2020/day/4)

use std::path::PathBuf;

use clap::Parser;

use rdcl_aoc_core::input::InputReader;

use crate::passport::Passport;

mod passport;

#[derive(Parser, Debug)]
#[clap(about = "The solution for advent of code 2020, day 4")]
struct Args {
    /// The file which contains the puzzle input.
    input: PathBuf,
}

fn main() {
    let args: Args = Args::parse();
    let lines = InputReader::from(args.input);
    let Counts(nr_complete, nr_valid) = process(lines.read_lines());

    println!("There are {} complete passports", nr_complete);
    println!("There are {} valid passports", nr_valid);
}

struct Counts(usize, usize);

impl Counts {
    fn update(&mut self, passport: Passport) {
        if passport.is_complete() {
            self.0 += 1;
        }
        if passport.is_valid() {
            self.1 += 1;
        }
    }
}

fn process<T>(lines: T) -> Counts
where
    T: Iterator<Item = String>,
{
    let mut counts = Counts(0, 0);
    let mut passport = Passport::default();

    for line in lines {
        if line.is_empty() {
            counts.update(passport);
            passport = Passport::default();
        } else {
            for part in line.split(' ') {
                let v = Some(part[4..].to_string());
                match &part[..3] {
                    "byr" => passport.byr = v,
                    "iyr" => passport.iyr = v,
                    "eyr" => passport.eyr = v,
                    "hgt" => passport.hgt = v,
                    "hcl" => passport.hcl = v,
                    "ecl" => passport.ecl = v,
                    "pid" => passport.pid = v,
                    "cid" => passport.cid = v,
                    _ => panic!("Invalid input: {}", part),
                };
            }
        }
    }
    counts.update(passport);

    counts
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data(file: &str) -> impl Iterator<Item = String> {
        InputReader::from(PathBuf::from(format!("./src/day04/{}", file))).read_lines()
    }

    #[test]
    fn test_nr_complete() {
        let Counts(nr_complete, _) = process(test_data("test_complete.txt"));
        assert_eq!(nr_complete, 2);
    }

    #[test]
    fn test_nr_valid() {
        let Counts(_, nr_valid) = process(test_data("test_valid.txt"));
        assert_eq!(nr_valid, 4);
    }
}
