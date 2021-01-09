use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::{ParseError, WithOrExit};

fn main() {
    let args = get_args(&["<input file>"], 1);
    let lengths = parse_input_v1(&args[1]).or_exit_with(1);

    let sparse_hash = compute_sparse_hash(&lengths, 256, 1);
    println!(
        "The product of the first two numbers after tying the knots is {}.",
        (sparse_hash[0] as u32) * (sparse_hash[1] as u32)
    );

    let lengths = parse_input_v2(&args[1]).or_exit_with(1);
    println!("The knot hash is {}.", compute_knot_hash(&lengths))
}

fn compute_knot_hash(lengths: &[u8]) -> String {
    let sparse_hash = compute_sparse_hash(&lengths, 256, 64);
    let dense_hash = convert_to_dense_hash(sparse_hash);
    hex::encode(dense_hash)
}

fn compute_sparse_hash(lengths: &[u8], rope_length: usize, rounds: usize) -> VecDeque<u8> {
    let mut rope = VecDeque::with_capacity(rope_length);
    for i in 0..rope_length {
        rope.push_back(i as u8);
    }

    let mut current_position: usize = 0;
    let mut skip_size: usize = 0;

    for _ in 0..rounds {
        for &length in lengths {
            let length = length as usize;
            for i in 0..length / 2 {
                rope.swap(i, length - i - 1);
            }
            let offset = (length + skip_size) % rope_length;
            current_position = (current_position + offset) % rope_length;
            skip_size += 1;
            rope.rotate_left(offset);
        }
    }
    rope.rotate_right(current_position);
    rope
}

fn convert_to_dense_hash(sparse_hash: VecDeque<u8>) -> Vec<u8> {
    let mut dense_hash = vec![];
    let mut ch = 0;
    for (idx, b) in sparse_hash.iter().enumerate() {
        if idx != 0 && idx % 16 == 0 {
            dense_hash.push(ch);
            ch = 0;
        }
        ch ^= *b;
    }
    dense_hash.push(ch);
    dense_hash
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tie_knots() {
        let mut expected: VecDeque<u8> = VecDeque::new();
        expected.push_back(3);
        expected.push_back(4);
        expected.push_back(2);
        expected.push_back(1);
        expected.push_back(0);
        assert_eq!(compute_sparse_hash(&[3, 4, 1, 5], 5, 1), expected);
    }
}
