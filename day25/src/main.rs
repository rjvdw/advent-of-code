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
    let public_keys = (public_keys[0], public_keys[1]);

    let loop_size = determine_loop_size(public_keys.0);
    let encryption_key = transform(public_keys.1, loop_size);

    println!("The encryption key is {}", encryption_key);
}

fn determine_loop_size(public_key: u64) -> u64 {
    let mut loop_size = 0;
    let mut transformed = INITIAL_NUMBER;
    while transformed != public_key {
        loop_size += 1;
        transformed = (transformed * SUBJECT_NUMBER) % MODULUS;
    }
    loop_size
}

fn transform(number: u64, loop_size: u64) -> u64 {
    let mut transformed = INITIAL_NUMBER;
    for _ in 0..loop_size {
        transformed = (transformed * number) % MODULUS;
    }
    transformed
}

#[cfg(test)]
mod tests {
    use super::*;

    const CARD_PUBLIC_KEY: u64 = 5764801;
    const DOOR_PUBLIC_KEY: u64 = 17807724;
    const CARD_LOOP_SIZE: u64 = 8;
    const DOOR_LOOP_SIZE: u64 = 11;
    const ENCRYPTION_KEY: u64 = 14897079;

    #[test]
    fn test_card_loop_size() {
        assert_eq!(determine_loop_size(CARD_PUBLIC_KEY), CARD_LOOP_SIZE)
    }

    #[test]
    fn test_door_loop_size() {
        assert_eq!(determine_loop_size(DOOR_PUBLIC_KEY), DOOR_LOOP_SIZE)
    }

    #[test]
    fn test_transform_card() {
        assert_eq!(transform(CARD_PUBLIC_KEY, DOOR_LOOP_SIZE), ENCRYPTION_KEY)
    }

    #[test]
    fn test_transform_door() {
        assert_eq!(transform(DOOR_PUBLIC_KEY, CARD_LOOP_SIZE), ENCRYPTION_KEY)
    }
}
