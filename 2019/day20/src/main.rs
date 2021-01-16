use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;

use maze::maze::Maze;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let file = File::open(&args[1]).or_exit_with(1);
    let maze = Maze::parse_input(file).or_exit_with(1);

    match maze.find_shortest_route() {
        Some(steps) => println!("The shortest route from AA to ZZ takes {} steps.", steps),
        None => eprintln!("There is no route from AA to ZZ."),
    }

    match maze.as_recursive_maze().find_shortest_route() {
        Some(steps) => println!(
            "In the recursive maze, the shortest route from AA to ZZ takes {} steps.",
            steps
        ),
        None => eprintln!("There is no route from AA to ZZ."),
    }
}
