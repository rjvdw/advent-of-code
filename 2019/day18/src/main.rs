use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;

use crate::cave::four_way::FourWayCave;
use crate::cave::simple::SimpleCave;
use crate::cave::Cave;

mod cave;

fn main() {
    let args = get_args(&["<input file>"], 1);

    let file = File::open(&args[1]).or_exit_with(1);
    let simple_cave = SimpleCave::parse(file).or_exit_with(1);

    let file = File::open(&args[1]).or_exit_with(1);
    let four_way_cave = FourWayCave::parse(file).or_exit_with(1);

    // println!("simple cave:");
    // println!("{}", simple_cave);

    // println!("four way cave:");
    // println!("{}", four_way_cave);

    match simple_cave.find_shortest_path() {
        Some(distance) => println!(
            "The shortest path in the simple maze that finds all keys has length {}.",
            distance
        ),
        None => eprintln!("There is no path that finds all keys."),
    }
    match four_way_cave.find_shortest_path() {
        Some(distance) => println!(
            "The shortest path in the four-way maze that finds all keys has length {}.",
            distance
        ),
        None => eprintln!("There is no path that finds all keys."),
    }
}
