mod input_record;

extern crate helpers;

use std::env;
use std::process::exit;
use helpers::{read_input, handle_result};
use input_record::InputRecord;

/// https://adventofcode.com/2020/day/2
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <input file>", &args[0]);
        exit(1);
    }

    let path = &args[1];
    let values: Vec<InputRecord> = handle_result(read_input(path));

    println!("Number of valid passwords according to old job: {}", values.iter()
        .filter(|v| v.valid_according_to_old_job()).count());
    println!("Number of valid passwords according to corporate policy: {}", values.iter()
        .filter(|v| v.valid_according_to_corporate_policy()).count());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input1 = "1-3 a: abcde".parse::<InputRecord>().unwrap();
        let input2 = "1-3 b: cdefg".parse::<InputRecord>().unwrap();
        let input3 = "2-9 c: ccccccccc".parse::<InputRecord>().unwrap();

        assert!(input1.valid_according_to_old_job());
        assert!(!input2.valid_according_to_old_job());
        assert!(input3.valid_according_to_old_job());
    }

    #[test]
    fn test_part_2() {
        let input1 = "1-3 a: abcde".parse::<InputRecord>().unwrap();
        let input2 = "1-3 b: cdefg".parse::<InputRecord>().unwrap();
        let input3 = "2-9 c: ccccccccc".parse::<InputRecord>().unwrap();

        assert!(input1.valid_according_to_old_job());
        assert!(!input2.valid_according_to_corporate_policy());
        assert!(!input3.valid_according_to_corporate_policy());
    }
}
