extern crate rdcl_aoc_helpers;

use std::process::exit;
use std::{env, iter};

use rdcl_aoc_helpers::error::WithOrExit;
use rdcl_aoc_helpers::part::Part;

const BASE: usize = 10;

/// https://adventofcode.com/2020/day/23
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 5 {
        eprintln!(
            "Usage: {} <cups> <nr moves> <nr cups> <part1|part2>",
            &args[0]
        );
        exit(1);
    }

    let initial_labeling = args[1].parse::<usize>().or_exit_with(1);
    let nr_moves = args[2].replace('_', "").parse::<usize>().or_exit_with(1);
    let nr_cups = args[3].replace('_', "").parse::<usize>().or_exit_with(1);
    let part = args[4].parse::<Part>().or_exit_with(1);

    if initial_labeling == 0 {
        eprintln!("I cannot play without cups!");
        exit(1);
    }

    let (cups, mut linked_list) = cups_from_labeling(initial_labeling, nr_cups);
    play(&mut linked_list, cups[0] - 1, nr_moves, nr_cups);

    println!(
        "[{}] Starting with {} (nr cups: {}), after {} moves, the result is {}.",
        part,
        initial_labeling,
        nr_cups,
        nr_moves,
        as_answer(&linked_list, &part)
    );
}

fn cups_from_labeling(mut labeling: usize, nr_cups: usize) -> (Vec<usize>, Vec<usize>) {
    let mut cups: Vec<usize> = Vec::new();
    while labeling != 0 {
        cups.push(labeling % BASE);
        labeling /= BASE;
    }
    cups = cups.iter().rev().cloned().collect();
    let labeling_len = cups.len();
    while cups.len() < nr_cups {
        cups.push(cups.len() + 1);
    }

    let mut linked_list: Vec<usize> = Vec::with_capacity(nr_cups);
    linked_list.extend(1..=nr_cups);
    let last_link = if labeling_len == nr_cups {
        cups.first().cloned().unwrap()
    } else {
        nr_cups + 1
    };
    for (i, &x) in cups.iter().enumerate() {
        let idx = x - 1;
        let cup = cups.get(i + 1).cloned().unwrap_or(last_link) - 1;
        linked_list[idx] = cup;
    }
    if labeling_len != nr_cups {
        *linked_list.last_mut().unwrap() = cups[0] - 1;
    }

    (cups, linked_list)
}

fn as_answer(linked_list: &[usize], part: &Part) -> usize {
    match part {
        Part::One => iter::successors(linked_list.get(0), |&&cup| linked_list.get(cup))
            .take_while(|&&cup| cup != 0)
            .fold(0, |acc, &cup| BASE * acc + cup + 1),
        Part::Two => (linked_list[0] + 1) * (linked_list[linked_list[0]] + 1),
    }
}

fn play(linked_list: &mut [usize], mut current_cup: usize, moves: usize, nr_cups: usize) {
    for _ in 0..moves {
        // take three cups
        let cup_a = linked_list[current_cup];
        let cup_b = linked_list[cup_a];
        let cup_c = linked_list[cup_b];

        let next_current_cup = linked_list[cup_c];

        // determine the destination
        let destination = iter::successors(Some(current_cup), |&cup| {
            Some(if cup == 0 { nr_cups - 1 } else { cup - 1 })
        })
        .skip(1)
        .find(|&cup| cup != cup_a && cup != cup_b && cup != cup_c)
        .unwrap();

        // place the cups
        let tmp = linked_list[destination];
        linked_list[current_cup] = next_current_cup;
        linked_list[destination] = cup_a;
        linked_list[cup_c] = tmp;

        // prepare for the next round
        current_cup = next_current_cup;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cups_from_labeling() {
        let labeling = 389125467;
        let actual = cups_from_labeling(labeling, 9);
        let expected = (
            vec![3, 8, 9, 1, 2, 5, 4, 6, 7],
            vec![1, 4, 7, 5, 3, 6, 2, 8, 0],
        );

        assert_eq!(actual, expected);
    }

    mod part1 {
        use super::*;

        const PART: Part = Part::One;

        #[test]
        fn test_play_10_moves() {
            let labeling = 389_125_467;
            let (cups, mut linked_list) = cups_from_labeling(labeling, 9);
            play(&mut linked_list, cups[0] - 1, 10, 9);
            let answer = as_answer(&linked_list, &PART);

            assert_eq!(answer, 92_658_374);
        }

        #[test]
        fn test_play_100_moves() {
            let labeling = 389_125_467;
            let (cups, mut linked_list) = cups_from_labeling(labeling, 9);
            play(&mut linked_list, cups[0] - 1, 100, 9);
            let answer = as_answer(&linked_list, &PART);

            assert_eq!(answer, 67_384_529);
        }
    }

    mod part2 {
        use super::*;

        const PART: Part = Part::Two;

        #[test]
        fn test_play() {
            let labeling = 389_125_467;
            let (cups, mut linked_list) = cups_from_labeling(labeling, 1_000_000);
            play(&mut linked_list, cups[0] - 1, 10_000_000, 1_000_000);
            let answer = as_answer(&linked_list, &PART);

            assert_eq!(answer, 149_245_887_792);
        }
    }
}
