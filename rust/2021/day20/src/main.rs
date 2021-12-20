extern crate rdcl_aoc_helpers;

use std::fs::File;
use std::io::{BufRead, BufReader};

use rdcl_aoc_helpers::args::get_args_repeating;
use rdcl_aoc_helpers::error::WithOrExit;

use crate::image::Image;

mod bounds;
mod image;
mod point;
mod shared_state;

const DEFAULT_MIN_REGION_SIZE: i64 = 500;

fn main() {
    let args = get_args_repeating(&["<input file>", "<steps> ?<nr threads> ?<region size>"], 1);

    let file = File::open(&args[1]).or_exit_with(1);
    let reader = BufReader::new(file).lines();
    let mut image = Image::parse(reader).or_exit_with(1);

    let steps = args[2].parse::<usize>().or_exit_with(1);

    if args.len() > 3 {
        let nr_threads = args[3].parse::<i64>().or_exit_with(1);
        let min_region_size = if args.len() > 4 {
            args[4].parse::<i64>().or_exit_with(1)
        } else {
            DEFAULT_MIN_REGION_SIZE
        };

        println!(
            "Running the solution on {} threads, with a minimal region size of {}.",
            nr_threads, min_region_size
        );
        for _ in 0..steps {
            image = image.next_mt(nr_threads, min_region_size);
        }
    } else {
        println!("Running the solution single threaded.");
        for _ in 0..steps {
            image = image.next();
        }
    }
    match image.count_lit_pixels() {
        Ok(nr) => println!("After {} steps, {} pixels are lit.", steps, nr),
        Err(nr) => println!(
            "After {} steps, an infinite number of pixels are lit ({} of which are within bounds).",
            steps, nr,
        ),
    }
}
