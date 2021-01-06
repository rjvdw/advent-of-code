use std::fs::File;
use std::thread;
use std::time::Duration;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;
use rdcl_aoc_helpers::input::WithReadMultiLines;
use termion::{clear, cursor};

use rail_map::RailMap;

fn main() {
    let args = get_args(&["<input file>", "<speed>"], 1);
    let mut rail_map = File::open(&args[1])
        .read_multi_lines::<RailMap>(1)
        .next()
        .or_exit_with(1);
    let speed = args[2].parse::<usize>().or_exit_with(1);

    rail_map.set_pretty_print(true);
    let sleep_time = Duration::from_millis(speed as u64);

    while rail_map.get_nr_carts_remaining() > 1 {
        print!("{}{}", clear::All, cursor::Goto(1, 1));
        println!("{}", rail_map);
        thread::sleep(sleep_time);
        rail_map.tick();
    }
    print!("{}{}", clear::All, cursor::Goto(1, 1));
    println!("{}", rail_map);
}
