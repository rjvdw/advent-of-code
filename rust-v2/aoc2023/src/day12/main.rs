//! The solution for [advent of code 2023, day 12](https://adventofcode.com/2023/day/12)

use std::path::PathBuf;
use std::str::FromStr;

use clap::Parser;

use rdcl_aoc_core::input::InputReader;

use crate::report::Record;

mod report;

const COPIES_WHEN_UNFOLDING: usize = 5;

#[derive(Parser, Debug)]
#[clap(about = "The solution for advent of code 2023, day 12")]
struct Args {
    /// The file which contains the puzzle input.
    input: PathBuf,
}

fn main() {
    let args: Args = Args::parse();
    let records = InputReader::from(args.input)
        .parse_lines(Record::from_str)
        .collect::<Vec<_>>();

    let mut folded = 0;
    let mut unfolded = 0;
    for record in records {
        // println!("Evaluating {record}");
        folded += record.count_correct_configurations();
        unfolded += record
            .unfold(COPIES_WHEN_UNFOLDING)
            .count_correct_configurations();
    }

    println!("Before unfolding, the sum of the possible arrangement counts is {folded}");
    println!("After unfolding, the sum of the possible arrangement counts is {unfolded}");
}
