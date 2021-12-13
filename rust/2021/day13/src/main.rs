extern crate rdcl_aoc_helpers;

use std::fs::File;
use std::io::{BufRead, BufReader};

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;

use crate::manual::Manual;

mod manual;

/// https://adventofcode.com/2021/day/13
fn main() {
    let args = get_args(&["<input file>"], 1);

    let file = File::open(&args[1]).or_exit_with(1);
    let lines = BufReader::new(file).lines();

    let mut manual = Manual::parse(lines).or_exit_with(1);
    println!(
        "Initially, there are {} dots visible in the manual.",
        manual.count_visible_dots()
    );

    let mut count = 0;
    while manual.nr_folds() > 0 {
        manual.fold();
        count += 1;
        println!(
            "After {} fold{}, there are {} dots visible in the manual.",
            count,
            if count == 1 { "" } else { "s" },
            manual.count_visible_dots(),
        );
    }

    println!("The manual now looks like this:");
    println!("{}", manual);
}
