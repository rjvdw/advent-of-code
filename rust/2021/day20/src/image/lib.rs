use std::fs::File;
use std::io::{BufRead, BufReader};

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;

use crate::image::Image;
use crate::mt::next::MultiThreadedNext;
use crate::st::next::SingleThreadedNext;

mod bounds;
mod image;
mod mt;
mod point;
#[cfg(test)]
mod shared_test;
mod st;

pub fn main(multi_threaded: bool) {
    let args = get_args(&["<input file>", "<steps>"], 1);

    let file = File::open(&args[1]).or_exit_with(1);
    let reader = BufReader::new(file).lines();
    let mut image = Image::parse(reader).or_exit_with(1);

    let steps = args[2].parse::<usize>().or_exit_with(1);

    if multi_threaded {
        println!("Running the solution on multiple threads.");
        for _ in 0..steps {
            image = MultiThreadedNext::next(image);
        }
    } else {
        println!("Running the solution on a single thread.");
        for _ in 0..steps {
            image = SingleThreadedNext::next(image);
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
