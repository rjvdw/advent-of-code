extern crate rdcl_aoc_helpers;

use std::fs::File;
use std::process::exit;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::input::WithReadLines;

const INITIAL_NUMBER: u64 = 1;
const MODULUS: u64 = 20201227;
const SUBJECT_NUMBER: u64 = 7;

fn main() {
    let args = get_args(&["<input file>"], 1);

    let public_keys = File::open(&args[1]).read_lines(1).collect::<Vec<u64>>();
    if public_keys.len() != 2 {
        eprintln!("Input file must contain exactly two lines.");
        exit(1);
    }
    println!(
        "The encryption key is {}",
        find_encryption_key(public_keys[0], public_keys[1])
    );
}

fn find_encryption_key(pub1: u64, pub2: u64) -> u64 {
    let mut t = INITIAL_NUMBER;
    let mut e1 = INITIAL_NUMBER;
    let mut e2 = INITIAL_NUMBER;

    while t != pub1 && t != pub2 {
        t = transform(t, SUBJECT_NUMBER);
        e1 = transform(e1, pub1);
        e2 = transform(e2, pub2);
    }

    if t == pub1 {
        e2
    } else {
        e1
    }
}

fn transform(nr: u64, subject: u64) -> u64 {
    (nr * subject) % MODULUS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(find_encryption_key(5_764_801, 17_807_724), 14_897_079)
    }
}
