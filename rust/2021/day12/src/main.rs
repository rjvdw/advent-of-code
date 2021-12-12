extern crate rdcl_aoc_helpers;

use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;
use rdcl_aoc_helpers::input::WithReadMultiLines;

use crate::cave_map::CaveMap;

mod cave_map;

/// https://adventofcode.com/2021/day/12
fn main() {
    let args = get_args(&["<input file>", "<max revisits>"], 1);

    let map = File::open(&args[1])
        .read_multi_lines::<CaveMap>(1)
        .next()
        .or_exit_with(1);
    let max_revisits = args[2].parse::<usize>().or_exit_with(1);

    println!(
        "There are {} paths that don't revisit a small cave more than {} times.",
        map.count_paths(max_revisits),
        max_revisits,
    );
}
