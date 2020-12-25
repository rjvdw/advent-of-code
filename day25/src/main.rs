extern crate helpers;

use std::env;
use std::process::exit;

use helpers::handle_result;
use helpers::read::read_input;

use crate::key_pair::KeyPair;

mod key_pair;

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
    KeyPair::new(pub1, pub2).find_encryption_key().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(find_encryption_key(5764801, 17807724), 14897079)
    }
}
