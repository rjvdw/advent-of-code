extern crate rdcl_aoc_helpers;

use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Write};

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;

use crate::risk_levels::RiskLevels;

mod risk_levels;

/// https://adventofcode.com/2021/day/15
fn main() {
    let args = get_args(&["<input file>"], 1);
    let file = File::open(&args[1]).or_exit_with(1);
    let lines = BufReader::new(file).lines();
    let risk_levels = RiskLevels::parse(lines).or_exit_with(1);

    print!("In the small cave: ");
    match risk_levels.find_optimal_path() {
        Some(score) => println!("The lowest possible risk score is {}.", score),
        None => eprintln!("There is no path through this cave."),
    }

    let transformed = risk_levels.transform();
    print!("In the big cave: ");
    io::stdout().flush().unwrap();
    match transformed.find_optimal_path() {
        Some(score) => println!("The lowest possible risk score is {}.", score),
        None => eprintln!("There is no path through this cave."),
    }
}
