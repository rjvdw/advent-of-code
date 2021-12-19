extern crate rdcl_aoc_helpers;

use std::fs::File;
use std::process::exit;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::input::WithReadLines;

use boarding_pass::BoardingPass;

mod boarding_pass;

fn main() {
    let args = get_args(&["<input file>"], 1);

    let mut seat_ids = File::open(&args[1])
        .read_lines::<BoardingPass>(1)
        .map(|r| r.get_seat_id())
        .collect::<Vec<u16>>();

    seat_ids.sort_unstable();

    if seat_ids.is_empty() {
        eprintln!("No seats found");
        exit(1);
    }

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
    fn test_example_1() {
        let record = BoardingPass::from_str("FBFBBFFRLR").unwrap();

        assert_eq!(record, BoardingPass(44, 5));
        assert_eq!(record.get_seat_id(), 357);
    }

    #[test]
    fn test_example_2() {
        let record = BoardingPass::from_str("BFFFBBFRRR").unwrap();

        assert_eq!(record, BoardingPass(70, 7));
        assert_eq!(record.get_seat_id(), 567);
    }

    #[test]
    fn test_example_3() {
        let record = BoardingPass::from_str("FFFBBBFRRR").unwrap();

        assert_eq!(record, BoardingPass(14, 7));
        assert_eq!(record.get_seat_id(), 119);
    }

    #[test]
    fn test_example_4() {
        let record = BoardingPass::from_str("BBFFBBFRLL").unwrap();

        assert_eq!(record, BoardingPass(102, 4));
        assert_eq!(record.get_seat_id(), 820);
    }
}
