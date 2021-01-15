use std::fs::File;
use std::thread;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;

use cave::Cave;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let file = File::open(&args[1]).or_exit_with(1);

    let cave_part_1 = Cave::parse(file).or_exit_with(1);
    let cave_part_2 = cave_part_1.with_four_entrances();

    let part1 = thread::spawn(move || match cave_part_1.find_quickest_route() {
        Some(distance) => println!(
            "The quickest route that reaches all keys has length {}.",
            distance
        ),
        None => eprintln!("There is no route that reaches all keys."),
    });

    let part2 = thread::spawn(move || match cave_part_2.find_quickest_route() {
        Some(distance) => println!(
            "The quickest route that reaches all keys has length {}.",
            distance
        ),
        None => eprintln!("There is no route that reaches all keys."),
    });

    part1.join().unwrap();
    part2.join().unwrap();
}
