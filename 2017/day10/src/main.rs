use std::fs::File;
use std::io::{BufRead, BufReader};

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::{ParseError, WithOrExit};

use shared::knot_hash::{compute_knot_hash, compute_sparse_hash};

fn main() {
    let args = get_args(&["<input file>"], 1);
    let lengths = parse_input_v1(&args[1]).or_exit_with(1);

    let sparse_hash = compute_sparse_hash(&lengths, 256, 1);
    println!(
        "The product of the first two numbers after tying the knots is {}.",
        (sparse_hash[0] as u32) * (sparse_hash[1] as u32)
    );

    let lengths = parse_input_v2(&args[1]).or_exit_with(1);
    println!("The knot hash is {:x?}.", compute_knot_hash(&lengths))
}

fn parse_input_v1(path: &str) -> Result<Vec<u8>, ParseError> {
    let mut numbers = Vec::new();
    let file = File::open(path)?;
    for line in BufReader::new(file).lines() {
        let line = line?;
        for nr in line.split(',') {
            numbers.push(nr.parse()?);
        }
    }
    Ok(numbers)
}

fn parse_input_v2(path: &str) -> Result<Vec<u8>, ParseError> {
    let mut numbers = Vec::new();
    let file = File::open(path)?;
    for line in BufReader::new(file).lines() {
        let line = line?;
        for ch in line.bytes() {
            numbers.push(ch);
        }
    }
    numbers.extend_from_slice(&[17, 31, 73, 47, 23]);
    Ok(numbers)
}
