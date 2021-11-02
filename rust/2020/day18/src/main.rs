extern crate rdcl_aoc_helpers;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::ParseError;

mod calc;
mod operator;
mod parse_mode;

/// https://adventofcode.com/2020/day/18
fn main() {
    let args = get_args(&["<input file>"], 1);

    match read(&args[1]) {
        Ok((sum_simple, sum_advanced)) => {
            println!("Sum (simple): {}", sum_simple);
            println!("Sum (advanced): {}", sum_advanced);
        }
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    }
}

fn read(path: &str) -> Result<(i64, i64), ParseError> {
    let file = File::open(path)?;
    let mut sum_simple = 0;
    let mut sum_advanced = 0;

    for line in BufReader::new(file).lines() {
        let expr = line?;
        sum_simple += calc::simple::evaluate(expr.as_str())?;
        sum_advanced += calc::advanced::evaluate(expr.as_str())?;
    }

    Ok((sum_simple, sum_advanced))
}
