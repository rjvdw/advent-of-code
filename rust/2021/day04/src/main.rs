extern crate rdcl_aoc_helpers;

use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::{ParseError, WithOrExit};
use rdcl_aoc_helpers::input::MappedMultiLines;
use rdcl_aoc_helpers::parse_error;

use crate::board::Board;

mod board;

/// https://adventofcode.com/2021/day/4
fn main() {
    let args = get_args(&["<input file>"], 1);

    let file = File::open(&args[1]).or_exit_with(1);
    let lines = BufReader::new(file).lines();
    let (numbers, mut boards) = parse_input(lines).or_exit_with(1);

    match play(&mut boards, &numbers) {
        Some((board, winning_nr, score)) => println!(
            "Board #{} won with {}, giving it a score of {}, so the final answer is {}.",
            board,
            winning_nr,
            score,
            (winning_nr as u32) * score,
        ),
        None => eprintln!("Noone won the game of squid bingo!"),
    }

    for board in boards.iter_mut() {
        board.reset();
    }

    match find_losing_board(boards, &numbers) {
        Some((board, winning_nr, score)) => println!(
            "The losing board is #{}, which will win with {}, giving it a score of {}, so the final answer is {}.",
            board,
            winning_nr,
            score,
            (winning_nr as u32) * score,
        ),
        None => eprintln!("Noone won the game of squid bingo!"),
    }
}

fn play(boards: &mut [Board], numbers: &[u8]) -> Option<(usize, u8, u32)> {
    for &number in numbers {
        for (idx, board) in boards.iter_mut().enumerate() {
            board.mark(number);
            if board.bingo() {
                return Some((idx, number, board.score()));
            }
        }
    }
    None
}

fn find_losing_board(mut boards: Vec<Board>, numbers: &[u8]) -> Option<(usize, u8, u32)> {
    let mut last_winning_board = None;
    let mut ignored = HashSet::new();
    for &number in numbers {
        for (idx, board) in boards.iter_mut().enumerate() {
            if !ignored.contains(&idx) {
                board.mark(number);
                if board.bingo() {
                    ignored.insert(idx);
                    last_winning_board = Some((idx, number, board.score()));
                }
            }
        }
    }
    last_winning_board
}

fn parse_input<T>(mut lines: T) -> Result<(Vec<u8>, Vec<Board>), ParseError>
where
    T: Iterator<Item = io::Result<String>>,
{
    let mut numbers: Vec<u8> = vec![];
    match lines.next() {
        Some(line) => {
            for nr in line?.split(',') {
                numbers.push(nr.parse()?);
            }
        }
        _ => return Err(parse_error!("Invalid input data")),
    };

    let boards = MappedMultiLines::from_iterator(lines.skip(1), 1).collect();

    Ok((numbers, boards))
}

#[cfg(test)]
mod tests {
    use grid::grid;

    use super::*;

    #[test]
    fn test_play() {
        let (numbers, mut boards) = get_test_data();
        assert_eq!(play(&mut boards, &numbers), Some((2, 24, 188)));
    }

    #[test]
    fn test_find_losing_board() {
        let (numbers, boards) = get_test_data();
        assert_eq!(find_losing_board(boards, &numbers), Some((1, 13, 148)));
    }

    fn get_test_data() -> (Vec<u8>, Vec<Board>) {
        let numbers = vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];
        let boards = vec![
            Board::new(grid![
                [22, 13, 17, 11,  0]
                [ 8,  2, 23,  4, 24]
                [21,  9, 14, 16,  7]
                [ 6, 10,  3, 18,  5]
                [ 1, 12, 20, 15, 19]
            ]),
            Board::new(grid![
                [ 3, 15,  0,  2, 22]
                [ 9, 18, 13, 17,  5]
                [19,  8,  7, 25, 23]
                [20, 11, 10, 24,  4]
                [14, 21, 16, 12,  6]
            ]),
            Board::new(grid![
                [14, 21, 17, 24,  4]
                [10, 16, 15,  9, 19]
                [18,  8, 23, 26, 20]
                [22, 11, 13,  6,  5]
                [ 2,  0, 12,  3,  7]
            ]),
        ];
        (numbers, boards)
    }
}
