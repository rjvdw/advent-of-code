use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;
use rdcl_aoc_helpers::input::WithReadMultiLines;

use crate::grid::Grid;

mod grid;

fn main() {
    let args = get_args(&["<input file>", "<steps>"], 1);
    let starting_grid = File::open(&args[1])
        .read_multi_lines::<Grid>(1)
        .next()
        .or_exit_with(1);
    let steps = args[2].parse::<usize>().or_exit_with(1);

    let mut grid = starting_grid.next();
    for _ in 1..steps {
        grid = grid.next();
    }

    println!(
        "After {} steps, there are {} lights turned on.",
        steps,
        grid.count_active_cells()
    );

    let mut grid = starting_grid;
    grid.with_stuck_corners = true;
    for _ in 0..steps {
        grid = grid.next();
    }

    println!(
        "If the corners are stuck, then after {} steps, there are {} lights turned on.",
        steps,
        grid.count_active_cells()
    );
}
