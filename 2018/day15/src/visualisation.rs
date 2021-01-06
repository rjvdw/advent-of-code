use std::fs::File;
use std::thread;
use std::time::Duration;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;
use rdcl_aoc_helpers::input::WithReadMultiLines;
use termion::{clear, cursor};

use combat::cave::Cave;
use combat::faction::Faction;

fn main() {
    let args = get_args(
        &[
            "<input file>",
            "<elves attack power>",
            "<goblins attack power>",
            "<speed>",
        ],
        1,
    );
    let mut cave = File::open(&args[1])
        .read_multi_lines::<Cave>(1)
        .next()
        .or_exit_with(1);
    let elves_attack_power = args[2].parse::<usize>().or_exit_with(1);
    let goblins_attack_power = args[3].parse::<usize>().or_exit_with(1);
    let speed = args[4].parse::<usize>().or_exit_with(1);

    cave.set_attack_power(Faction::Elf, elves_attack_power);
    cave.set_attack_power(Faction::Goblin, goblins_attack_power);

    let sleep_time = Duration::from_millis(speed as u64);

    let mut outcome = None;
    while outcome.is_none() {
        print!("{}{}", clear::All, cursor::Goto(1, 1));
        println!("{}", cave);
        thread::sleep(sleep_time);
        outcome = cave.take_turns();
    }
    print!("{}{}", clear::All, cursor::Goto(1, 1));
    println!("{}", cave);
}
