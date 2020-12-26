extern crate rdcl_aoc_helpers;

use std::fs::File;
use std::process::exit;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;
use rdcl_aoc_helpers::input::WithReadMultiLines;

use seat_layout::SeatLayout;

mod cardinal_direction;
mod seat_layout;
#[cfg(test)]
mod test_helpers;

/// https://adventofcode.com/2020/day/11
fn main() {
    let args = get_args(&["<input file>", "<view distance>", "<seat threshold>"], 1);

    let mut input = File::open(&args[1]).read_multi_lines::<SeatLayout>(1);
    let view_distance = args[2].parse::<usize>().or_exit_with(1);
    let seat_threshold = args[3].parse::<usize>().or_exit_with(1);
    if let Some(state) = input.next() {
        println!(
            "Number of occupied seats in the final state: {}",
            solve(state, view_distance, seat_threshold)
        );
    } else {
        eprintln!("Failed to process input");
        exit(1);
    }
}

fn solve(mut state: SeatLayout, view_distance: usize, seat_threshold: usize) -> usize {
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
#[rustfmt::skip::macros(vec)]
mod tests {
    use crate::test_helpers::get_input;

    use super::*;

    #[test]
    fn test_1() {
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
