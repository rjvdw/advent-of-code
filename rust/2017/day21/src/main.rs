use std::fs::File;

use grid::grid;
use grid::Grid;
use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;

use crate::parse::{parse_input, Mapping};

mod parse;
mod three_by_three;
mod two_by_two;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let file = File::open(&args[1]).or_exit_with(1);
    let enhancement_rules = parse_input(file).or_exit_with(1);

    // println!("{:#?}", enhancement_rules);

    println!(
        "After 5 iterations, there are {} pixels turned on.",
        count_pixels(do_art(&enhancement_rules, 5))
    );

    println!(
        "After 18 iterations, there are {} pixels turned on.",
        count_pixels(do_art(&enhancement_rules, 18))
    );
}

fn count_pixels(painting: Grid<u8>) -> usize {
    bytecount::count(painting.flatten(), 1)
}

fn do_art(enhancement_rules: &[Mapping<u8>], iterations: usize) -> Grid<u8> {
    let mut painting: Grid<u8> = grid![
        [0,1,0]
        [0,0,1]
        [1,1,1]
    ];

    for _ in 1..=iterations {
        if painting.cols() % 2 == 0 {
            painting = enhance(enhancement_rules, 2, painting);
        } else {
            painting = enhance(enhancement_rules, 3, painting);
        }
    }

    painting
}

fn enhance(enhancement_rules: &[Mapping<u8>], square_size: usize, painting: Grid<u8>) -> Grid<u8> {
    let square_count = painting.cols() / square_size;
    let new_size = (square_size + 1) * square_count;
    let mut enhanced = Grid::from_vec(vec![0; new_size * new_size], new_size);
    let mut square = Grid::from_vec(vec![0; square_size * square_size], square_size);

    // for every square in the painting:
    for j in 0..square_count {
        for i in 0..square_count {
            // copy square from the painting
            for y in 0..square_size {
                for x in 0..square_size {
                    square[y][x] = painting[y + j * square_size][x + i * square_size];
                }
            }

            // find the new square
            let enhanced_square = enhancement_rules
                .iter()
                .find(|(left, _)| *left == square)
                .map(|(_, right)| right)
                .unwrap();

            // copy the new square in the new painting
            for y in 0..square_size + 1 {
                for x in 0..square_size + 1 {
                    enhanced[y + j * (square_size + 1)][x + i * (square_size + 1)] =
                        enhanced_square[y][x];
                }
            }
        }
    }

    enhanced
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn do_art_twice() {
        let test_input = vec!["../.# => ##./#../...", ".#./..#/### => #..#/..../..../#..#"];
        let test_input = test_input.join("\n");
        let enhancement_rules = parse_input(test_input.as_bytes()).unwrap();

        let painting = do_art(&enhancement_rules, 2);
        let expected: Grid<u8> = grid![
            [1, 1, 0, 1, 1, 0]
            [1, 0, 0, 1, 0, 0]
            [0, 0, 0, 0, 0, 0]
            [1, 1, 0, 1, 1, 0]
            [1, 0, 0, 1, 0, 0]
            [0, 0, 0, 0, 0, 0]
        ];

        assert_eq!(painting, expected);
        assert_eq!(count_pixels(painting), 12);
    }
}
