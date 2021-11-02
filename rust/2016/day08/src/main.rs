use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;
use rdcl_aoc_helpers::input::WithReadLines;

use crate::instruction::Instruction;
use crate::screen::Screen;

mod instruction;
mod screen;

fn main() {
    let args = get_args(&["<input file>", "<screen width>", "<screen height>"], 1);
    let instructions = File::open(&args[1])
        .read_lines(1)
        .collect::<Vec<Instruction>>();
    let screen_width = args[2].parse::<usize>().or_exit_with(1);
    let screen_height = args[3].parse::<usize>().or_exit_with(1);
    let mut screen = Screen::new(screen_width, screen_height);

    run(&mut screen, &instructions);

    println!(
        "After running the instructions, {} pixels are lit.",
        screen.count_active_pixels()
    );

    println!("This is what the screen currently looks like:");
    println!("{}", screen);
}

fn run(screen: &mut Screen, instructions: &[Instruction]) {
    for instruction in instructions {
        instruction.execute(screen);
    }
}
