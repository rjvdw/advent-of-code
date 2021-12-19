extern crate rdcl_aoc_helpers;

use std::fs::File;
use std::io::{BufRead, BufReader};

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;

use crate::target_area::TargetArea;

mod target_area;

fn main() {
    let args = get_args(&["<input file>"], 1);

    let file = File::open(&args[1]).or_exit_with(1);
    let reader = BufReader::new(file).lines();
    let target_area = TargetArea::parse(reader).or_exit_with(1);

    println!(
        "The maximal height that can be reached is {}.",
        target_area.find_max_height(),
    );

    println!(
        "There are {} possible initial velocities.",
        target_area.find_all_valid_initial_velocities().len(),
    );
}
