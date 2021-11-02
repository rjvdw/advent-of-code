//! 2D visualisation of the x and y coordinates of the moons as they move around.

use std::collections::HashMap;
use std::fs::File;
use std::ops::RangeInclusive;
use std::thread;
use std::time::Duration;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::{ParseError, WithOrExit};
use rdcl_aoc_helpers::input::WithReadLines;
use termion::{clear, cursor};

use moon::Moon;

fn main() {
    let args = get_args(
        &[
            "<input file>",
            "<x_min>..<x_max>",
            "<y_min>..<y_max>",
            "<speed>",
        ],
        1,
    );
    let mut moons: Vec<Moon> = File::open(&args[1]).read_lines(1).collect();
    let x_range = parse_range(&args[2]).or_exit_with(1);
    let y_range = parse_range(&args[3]).or_exit_with(1);
    let speed = args[4].parse::<u64>().or_exit_with(1);
    let sleep_time = Duration::from_millis(speed);

    for t in 1.. {
        print_map(t, &moons, x_range.clone(), y_range.clone());
        moons = moons.iter().map(|moon| moon.update_moon(&moons)).collect();
        thread::sleep(sleep_time);
    }
}

fn parse_range(r: &str) -> Result<RangeInclusive<i64>, ParseError> {
    let mut range = (0, 0);
    for (idx, part) in r.split("..").enumerate() {
        match idx {
            0 => {
                range.0 = part.parse()?;
            }
            1 => {
                range.1 = part.parse()?;
            }
            _ => {
                panic!("Invalid input: {}", r);
            }
        }
    }
    Ok(range.0..=range.1)
}

fn print_map<X, Y>(t: usize, moons: &[Moon], x_range: X, y_range: Y)
where
    X: Iterator<Item = i64> + Clone,
    Y: Iterator<Item = i64> + Clone,
{
    println!("{}{}t={}", clear::All, cursor::Goto(1, 1), t);
    let mut positions = HashMap::new();
    for (idx, moon) in moons.iter().enumerate() {
        let (x, y, _) = moon.get_position();
        positions.insert((x, y), ((idx as u8) + b'A') as char);
    }
    print!("+");
    for _ in x_range.clone() {
        print!("-");
    }
    println!("+");
    for y in y_range {
        print!("|");
        for x in x_range.clone() {
            if let Some(ch) = positions.get(&(x, y)) {
                print!("{}", ch);
            } else {
                print!(" ");
            }
        }
        println!("|");
    }
    print!("+");
    for _ in x_range {
        print!("-");
    }
    println!("+");
}
