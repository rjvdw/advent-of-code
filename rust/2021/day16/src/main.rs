extern crate rdcl_aoc_helpers;

use std::fs::File;
use std::io::{BufRead, BufReader};

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;

use crate::packet::Packet;

mod packet;

fn main() {
    let args = get_args(&["<input file>"], 1);

    let file = File::open(&args[1]).or_exit_with(1);
    let reader = BufReader::new(file).lines();
    let packet = Packet::parse_input(reader).or_exit_with(1);

    println!("The sum of the versions is {}.", packet.sum_versions());
    println!("The transmission evaluates to {}.", packet.eval());
}
