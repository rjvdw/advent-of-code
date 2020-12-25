extern crate rdcl_aoc_helpers;

use std::env;
use std::process::exit;

use input_record::InputRecord;
use rdcl_aoc_helpers::read::read_input;

mod input_record;

/// https://adventofcode.com/2020/day/5
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <input file>", &args[0]);
        exit(1);
    }

    let path = &args[1];
    let values: Vec<InputRecord> = read_input(path).unwrap();

    if values.is_empty() {
        eprintln!("No seats found");
        exit(1);
    }

    let mut seat_ids = values.iter().map(|r| r.get_seat_id()).collect::<Vec<u16>>();
    seat_ids.sort_unstable();

    println!("highest seat ID: {}", seat_ids.last().unwrap());

    println!("the following IDs are missing:");
    let mut idx = *seat_ids.first().unwrap();
    for id in seat_ids {
        if idx != id {
            println!("  {}", idx);
        }
        idx = id + 1;
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_1() {
        let record = InputRecord::from_str("FBFBBFFRLR").unwrap();

        assert_eq!(record.row, 44);
        assert_eq!(record.col, 5);
        assert_eq!(record.get_seat_id(), 357);
    }

    #[test]
    fn test_2() {
        let record = InputRecord::from_str("BFFFBBFRRR").unwrap();

        assert_eq!(record.row, 70);
        assert_eq!(record.col, 7);
        assert_eq!(record.get_seat_id(), 567);
    }

    #[test]
    fn test_3() {
        let record = InputRecord::from_str("FFFBBBFRRR").unwrap();

        assert_eq!(record.row, 14);
        assert_eq!(record.col, 7);
        assert_eq!(record.get_seat_id(), 119);
    }

    #[test]
    fn test_4() {
        let record = InputRecord::from_str("BBFFBBFRLL").unwrap();

        assert_eq!(record.row, 102);
        assert_eq!(record.col, 4);
        assert_eq!(record.get_seat_id(), 820);
    }
}
