use std::fs::File;
use std::thread;
use std::time::Duration;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;
use termion::{clear, cursor};

use game_of_life::grid::Grid;

fn main() {
    let args = get_args(&["<input file>", "<speed>"], 1);
    let file = File::open(&args[1]).or_exit_with(1);
    let mut grid = Grid::parse(file).or_exit_with(1);
    let speed = args[2].parse::<u64>().or_exit_with(1);
    let sleep = Duration::from_millis(speed);

    loop {
        print!("{}{}", clear::All, cursor::Goto(1, 1));
        println!(
            "Biodiversity rating: {}",
            grid.calculate_biodiversity_rating()
        );
        println!("{}", grid);
        thread::sleep(sleep);
        grid = grid.tick();
    }
}
