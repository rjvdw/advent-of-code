use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;
use rdcl_aoc_helpers::input::WithReadMultiLines;

use cave::Cave;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let mut cave = File::open(&args[1])
        .read_multi_lines::<Cave>(1)
        .next()
        .or_exit_with(1);

    println!(
        "The risk level for this cave is {}.",
        cave.compute_risk_level()
    );

    match cave.find_fastest_path() {
        Some(time) => println!("It will take {} minutes to reach our target.", time),
        None => eprintln!("We cannot reach our target."),
    }
}
