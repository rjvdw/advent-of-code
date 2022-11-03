use std::collections::HashSet;
use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::direction::{CanTravel, Direction};
use rdcl_aoc_helpers::error::WithOrExit;

use shared::intcode;
use shared::intcode::parse::parse_input;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let file = File::open(&args[1]).or_exit_with(1);
    let program = parse_input(file).or_exit_with(1);

    println!(
        "After finishing the painting program, a total of {} tiles have been painted at least once.",
        paint(&program, 0).1
    );

    println!("If we start on a white tile, the following image appears:");
    display_image(paint(&program, 1).0);
}

fn display_image(painted: HashSet<(i64, i64)>) {
    let mut min_x = i64::MAX;
    let mut max_x = i64::MIN;
    let mut min_y = i64::MAX;
    let mut max_y = i64::MIN;

    for &(x, y) in &painted {
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
            if painted.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn paint(program: &intcode::Program, starting_color: i64) -> (HashSet<(i64, i64)>, usize) {
    let mut program = program.clone();
    let mut seen: HashSet<(i64, i64)> = HashSet::new();
    let mut painted: HashSet<(i64, i64)> = HashSet::new();
    let mut current_position = (0, 0);
    let mut direction = Direction::Up;

    if starting_color == 1 {
        painted.insert(current_position);
    }

    while !program.has_halted() {
        let color = i64::from(painted.contains(&current_position));
        program.send_message(color);
        program.run();

        seen.insert(current_position);
        match program.receive_message() {
            Some(0) => {
                painted.remove(&current_position);
            }
            Some(1) => {
                painted.insert(current_position);
            }
            _ => unreachable!(),
        }

        direction = match program.receive_message() {
            Some(0) => direction.turn_left(),
            Some(1) => direction.turn_right(),
            _ => unreachable!(),
        };

        current_position = direction.travel(&current_position);
    }

    (painted, seen.len())
}
