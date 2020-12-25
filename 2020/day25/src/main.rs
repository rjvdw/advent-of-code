extern crate helpers;

use std::env;
use std::process::exit;

use helpers::handle_result;
use helpers::read::read_input;

const INITIAL_NUMBER: u64 = 1;
const MODULUS: u64 = 20201227;
const SUBJECT_NUMBER: u64 = 7;

/// https://adventofcode.com/2020/day/25
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <input file>", &args[0]);
        exit(1);
    }

    let public_keys = handle_result(read_input::<u64>(&args[1]));
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
