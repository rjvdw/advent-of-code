use std::collections::{HashSet, VecDeque};
use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;

use shared::intcode;
use shared::intcode::parse::parse_input;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let file = File::open(&args[1]).or_exit_with(1);
    let program = parse_input(file).or_exit_with(1);

    match find_shortest_path(program) {
        Some(end_state) => {
            let time = flood(end_state.program, end_state.oxygen_system, end_state.walls);

            println!(
                "The shortest path to the oxygen system has length {}.",
                end_state.distance
            );

            println!(
                "It will take {} minutes to fully fill the area with oxygen.",
                time
            )
        }
        None => eprintln!("There is no path to the destination."),
    }
}

fn flood(
    program: intcode::Program,
    oxygen_system: (i64, i64),
    mut walls: HashSet<(i64, i64)>,
) -> usize {
    let mut seen: HashSet<(i64, i64)> = HashSet::new();
    seen.insert(oxygen_system);
    for &wall in &walls {
        seen.insert(wall);
    }
    let mut positions: VecDeque<((i64, i64), usize, intcode::Program)> = VecDeque::new();
    positions.push_back((oxygen_system, 0, program));
    let mut max_distance = 0;

    while let Some((position, mut distance, program)) = positions.pop_front() {
        distance += 1;
        for &(neighbour, direction) in &neighbours(position) {
            if !seen.contains(&neighbour) {
                let mut program = program.clone();
                program.send_message(direction);
                program.run();
                match program.receive_message() {
                    Some(0) => {
                        walls.insert(neighbour);
                        seen.insert(neighbour);
                    }
                    Some(1) => {
                        seen.insert(neighbour);
                        positions.push_back((neighbour, distance, program));
                        if distance > max_distance {
                            max_distance = distance;
                        }
                    }
                    x => unreachable!("Unreachable arm: {:?}", x),
                }
            }
        }
    }

    paint_map(seen, walls, Some(oxygen_system));

    max_distance
}

struct ShortestPathEndState {
    oxygen_system: (i64, i64),
    distance: usize,
    program: intcode::Program,
    walls: HashSet<(i64, i64)>,
}

fn find_shortest_path(program: intcode::Program) -> Option<ShortestPathEndState> {
    let mut seen: HashSet<(i64, i64)> = HashSet::new();
    seen.insert((0, 0));
    let mut positions: VecDeque<((i64, i64), usize, intcode::Program)> = VecDeque::new();
    positions.push_back(((0, 0), 0, program));

    let mut walls: HashSet<(i64, i64)> = HashSet::new();

    while let Some((position, mut distance, program)) = positions.pop_front() {
        distance += 1;
        for &(neighbour, direction) in &neighbours(position) {
            if !seen.contains(&neighbour) {
                let mut program = program.clone();
                program.send_message(direction);
                program.run();
                match program.receive_message() {
                    Some(0) => {
                        walls.insert(neighbour);
                        seen.insert(neighbour);
                    }
                    Some(1) => {
                        seen.insert(neighbour);
                        positions.push_back((neighbour, distance, program));
                    }
                    Some(2) => {
                        return Some(ShortestPathEndState {
                            oxygen_system: neighbour,
                            distance,
                            program,
                            walls,
                        });
                    }
                    x => unreachable!("Unreachable arm: {:?}", x),
                }
            }
        }
    }

    None
}

fn neighbours((x, y): (i64, i64)) -> [((i64, i64), i64); 4] {
    [
        ((x, y - 1), 1), // north
        ((x, y + 1), 2), // south
        ((x - 1, y), 3), // west
        ((x + 1, y), 4), // east
    ]
}

fn paint_map(
    seen: HashSet<(i64, i64)>,
    walls: HashSet<(i64, i64)>,
    destination: Option<(i64, i64)>,
) {
    let mut min_x = i64::MAX;
    let mut max_x = i64::MIN;
    let mut min_y = i64::MAX;
    let mut max_y = i64::MIN;

    for &(x, y) in &seen {
        if x < min_x {
            min_x = x;
        }
        if x > max_x {
            max_x = x;
        }
        if y < min_y {
            min_y = y;
        }
        if y > max_y {
            max_y = y;
        }
    }

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if destination == Some((x, y)) {
                print!("()");
            } else if (x, y) == (0, 0) {
                print!("><");
            } else if walls.contains(&(x, y)) {
                print!("##");
            } else if seen.contains(&(x, y)) {
                print!("  ");
            } else {
                print!("##");
            }
        }
        println!();
    }
}
