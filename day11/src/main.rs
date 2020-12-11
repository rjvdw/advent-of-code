extern crate helpers;

use std::env;
use std::process::exit;

use helpers::{handle_result, read_multiline_input};
use seat_layout::SeatLayout;

mod seat_layout;
#[cfg(test)]
mod test_helpers;

/// https://adventofcode.com/2020/day/11
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!(
            "Usage: {} <input file> <view distance>, <seat threshold>",
            &args[0]
        );
        exit(1);
    }

    let input: Vec<SeatLayout> = handle_result(read_multiline_input(&args[1]));
    let view_distance = handle_result(args[2].parse::<usize>());
    let seat_threshold = handle_result(args[3].parse::<usize>());
    if let Some(state) = input.first() {
        println!(
            "Number of occupied seats in the final state: {}",
            solve(state.clone(), view_distance, seat_threshold)
        );
    } else {
        eprintln!("Failed to process input");
        exit(1);
    }
}

fn solve(initial_state: SeatLayout, view_distance: usize, seat_threshold: usize) -> usize {
    let mut state = initial_state;
    loop {
        let (next, changed) = state.next(view_distance, seat_threshold);
        if changed {
            state = next;
        } else {
            return state.nr_of_occupied_seats();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::get_input;

    #[test]
    fn test_1() {
        #[rustfmt::skip]
        let state = get_input(vec![
            "L.LL.LL.LL",
            "LLLLLLL.LL",
            "L.L.L..L..",
            "LLLL.LL.LL",
            "L.LL.LL.LL",
            "L.LLLLL.LL",
            "..L.L.....",
            "LLLLLLLLLL",
            "L.LLLLLL.L",
            "L.LLLLL.LL",
        ]);

        assert_eq!(solve(state, 1, 4), 37);
    }

    #[test]
    fn test_2() {
        #[rustfmt::skip]
        let state = get_input(vec![
            "L.LL.LL.LL",
            "LLLLLLL.LL",
            "L.L.L..L..",
            "LLLL.LL.LL",
            "L.LL.LL.LL",
            "L.LLLLL.LL",
            "..L.L.....",
            "LLLLLLLLLL",
            "L.LLLLLL.L",
            "L.LLLLL.LL",
        ]);

        assert_eq!(solve(state, 0, 5), 26);
    }
}
