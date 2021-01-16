use std::fs::File;
use std::thread;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;

use cave::Cave;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let file = File::open(&args[1]).or_exit_with(1);

    let cave_part_1 = Cave::parse(file).or_exit_with(1);
    let cave_part_2 = cave_part_1.clone();

    let part1 = thread::spawn(
        move || match cave_part_1.find_quickest_route_by_yourself() {
            Some(distance) => println!(
                "[part 1] The quickest route that reaches all keys has length {}.",
                distance
            ),
            None => eprintln!("[part 1] There is no route that reaches all keys."),
        },
    );

    let part2 = thread::spawn(
        move || match cave_part_2.find_quickest_route_with_four_drones() {
            Some(distance) => println!(
                "[part 2] The quickest route that reaches all keys has length {}.",
                distance
            ),
            None => eprintln!("[part 2] There is no route that reaches all keys."),
        },
    );

    part1.join().unwrap();
    part2.join().unwrap();
}
