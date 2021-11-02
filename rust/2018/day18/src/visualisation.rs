use std::fs::File;
use std::thread;
use std::time::Duration;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;
use rdcl_aoc_helpers::input::WithReadMultiLines;
use termion::{clear, cursor};

use forest::Forest;

fn main() {
    let args = get_args(&["<input file>", "<steps>", "<speed>"], 1);
    let mut forest = File::open(&args[1])
        .read_multi_lines::<Forest>(1)
        .next()
        .or_exit_with(1);
    let steps = args[2].parse::<usize>().or_exit_with(1);
    let speed = args[3].parse::<usize>().or_exit_with(1);

    forest.set_print_colors(true);
    let sleep_time = Duration::from_millis(speed as u64);

    for i in 0..steps {
        print(i, &forest);
        forest.next_iteration();
        thread::sleep(sleep_time);
    }
    print(steps, &forest);
}

fn print(steps: usize, forest: &Forest) {
    print!("{}{}", clear::All, cursor::Goto(1, 1));
    println!("Time: {}:{:02} (steps={})", steps / 60, steps % 60, steps);
    println!("{}", forest);
}
