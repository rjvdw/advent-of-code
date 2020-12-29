use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;
use rdcl_aoc_helpers::input::{WithReadLines, WithReadMultiLines};

use crate::sue::Sue;
use crate::ticker_tape::TickerTape;

mod sue;
mod ticker_tape;

fn main() {
    let args = get_args(&["<input file>", "<ticker tape>"], 1);
    let sues = File::open(&args[1]).read_lines::<Sue>(1);
    let ticker_tape = File::open(&args[2])
        .read_multi_lines::<TickerTape>(1)
        .next()
        .or_exit_with(1)
        .as_map();

    for sue in sues {
        if sue.matches_p1(&ticker_tape) {
            println!(
                "Ignoring the outdated retroencabulator, {} matches all the criteria.",
                sue.get_name()
            );
        }

        if sue.matches_p2(&ticker_tape) {
            println!(
                "Adjusting for the outdated retroencabulator, {} matches all the criteria.",
                sue.get_name()
            );
        }
    }
}
