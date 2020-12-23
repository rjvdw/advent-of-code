mod traits_and_types;

extern crate helpers;

use std::collections::VecDeque;
use std::env;
use std::process::exit;

use crate::traits_and_types::{
    CupNumber, Labeling, Part, PickedUpCups, WithContainsCup, WithCupsCount,
};
use helpers::handle_result;

const BASE: Labeling = 10;

/// https://adventofcode.com/2020/day/23
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 5 {
        eprintln!("Usage: {} <cups> <nr moves> <cap> <part1|part2>", &args[0]);
        exit(1);
    }

    let initial_labeling = handle_result(args[1].parse::<Labeling>());
    let nr_moves = handle_result(args[2].replace('_', "").parse::<usize>());
    let cap = handle_result(args[3].replace('_', "").parse::<CupNumber>());
    let part = if args[4] == *"part1" {
        Part::One
    } else if args[4] == *"part2" {
        Part::Two
    } else {
        eprintln!("Invalid argument: {}", args[4]);
        exit(1);
    };

    if initial_labeling == 0 {
        eprintln!("I cannot play without cups!");
        exit(1);
    }

    println!(
        "Starting with {} (cap={}), after {} moves, the result is {}.",
        initial_labeling,
        cap,
        nr_moves,
        play(initial_labeling, nr_moves, cap, part),
    );
}

fn play(initial_labeling: Labeling, moves: usize, cap: CupNumber, part: Part) -> Labeling {
    let (mut cups, min, max) = convert_labeling_to_cups(initial_labeling, cap);

    for i in 0..moves {
        if i % 0b100000000000000 == 0 {
            println!("Progress: {:.2}%", 100f32 * (i as f32) / (moves as f32));
        }
        // println!("--- move {} ---", i + 1);
        // println!("cups: {:?}", cups);
        let current_cup = cups.pop_front().unwrap();
        // println!("current cup: {}", current_cup);
        let picked_up = take_cups(&mut cups);
        // println!("pick up: {:?}", picked_up);
        cups.push_front(current_cup);
        let mut destination = safe_decrement(current_cup, min, max);
        while picked_up.contains_cup(destination) {
            destination = safe_decrement(destination, min, max);
        }
        // println!("destination: {}", destination);
        place_cups(&mut cups, picked_up, destination);
        cups.rotate_left(1);
        // println!();
    }

    // println!("-- final --");
    // println!("cups: {:?}", cups);

    match part {
        Part::One => convert_cups_to_labeling(&cups, 1),
        Part::Two => get_product_of_next_n_cups(&cups, 2, 1),
    }
}

fn convert_labeling_to_cups(
    mut labeling: Labeling,
    cap: CupNumber,
) -> (VecDeque<CupNumber>, CupNumber, CupNumber) {
    let mut cups = VecDeque::new();
    let mut min = CupNumber::MAX;
    let mut max = CupNumber::MIN;
    while labeling > 0 {
        let cup = (labeling % BASE) as CupNumber;
        labeling /= BASE;

        cups.push_front(cup);
        if cup < min {
            min = cup;
        }
        if cup > max {
            max = cup;
        }
    }
    while cups.len() < (cap as usize) {
        max += 1;
        cups.push_back(max);
    }
    (cups, min, max)
}

fn convert_cups_to_labeling(cups: &VecDeque<CupNumber>, starting_cup: CupNumber) -> Labeling {
    let mut labeling = 0;

    if !cups.is_empty() {
        let mut cups = cups.clone();
        while let Some(cup) = cups.pop_front() {
            if cup == starting_cup {
                break;
            } else {
                cups.push_back(cup);
            }
        }
        while let Some(cup) = cups.pop_front() {
            labeling *= BASE;
            labeling += cup as Labeling;
        }
    }

    labeling
}

fn get_product_of_next_n_cups(cups: &VecDeque<CupNumber>, n: usize, after: CupNumber) -> Labeling {
    let mut product = 1;
    let mut index = find_next_cup_index(cups, after);
    for _ in 0..n {
        product *= cups[index];
        index = (index + 1) % cups.len();
    }
    product
}

fn take_cups(cups: &mut VecDeque<CupNumber>) -> PickedUpCups {
    (
        cups.pop_front().unwrap(),
        cups.pop_front().unwrap(),
        cups.pop_front().unwrap(),
    )
}

fn place_cups(cups: &mut VecDeque<CupNumber>, to_place: PickedUpCups, destination: CupNumber) {
    for _ in 0..PickedUpCups::LEN {
        cups.push_front(0);
    }
    for to_index in 0..cups.len() - PickedUpCups::LEN {
        let from_index = to_index + PickedUpCups::LEN;
        cups[to_index] = cups[from_index];
        if cups[from_index] == destination {
            cups[to_index + 1] = to_place.0;
            cups[to_index + 2] = to_place.1;
            cups[to_index + 3] = to_place.2;
            break;
        }
    }
}

fn find_next_cup_index(cups: &VecDeque<CupNumber>, cup: CupNumber) -> usize {
    (1 + cups.iter().position(|&c| c == cup).unwrap()) % cups.len()
}

fn safe_decrement(nr: CupNumber, min: CupNumber, max: CupNumber) -> CupNumber {
    if nr == min {
        max
    } else {
        nr - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_labeling_to_cups_with_low_cap() {
        let labeling = 6352714;
        let expected = (as_vec_deque(&[6, 3, 5, 2, 7, 1, 4]), 1, 7);

        let cups = convert_labeling_to_cups(labeling, 7);

        assert_eq!(cups, expected);
    }

    #[test]
    fn test_convert_labeling_to_cups_with_high_cap() {
        let labeling = 6352714;
        let expected = (
            as_vec_deque(&[
                6, 3, 5, 2, 7, 1, 4, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
            ]),
            1,
            20,
        );

        let cups = convert_labeling_to_cups(labeling, 20);

        assert_eq!(cups, expected);
    }

    #[test]
    fn test_convert_cups_to_labeling() {
        let cups = as_vec_deque(&[6, 3, 5, 2, 7, 1, 4]);
        let expected = 463527;

        let labeling = convert_cups_to_labeling(&cups, 1);

        assert_eq!(labeling, expected);
    }

    #[test]
    fn test_get_product_of_next_n_cups_middle() {
        let cups = as_vec_deque(&[6, 3, 4, 2, 1, 5, 7]);
        let expected = 35;

        let actual = get_product_of_next_n_cups(&cups, 2, 1);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_product_of_next_n_cups_end() {
        let cups = as_vec_deque(&[7, 6, 3, 4, 2, 1, 5]);
        let expected = 35;

        let actual = get_product_of_next_n_cups(&cups, 2, 1);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_take_cups() {
        let mut cups = as_vec_deque(&[6, 3, 5, 2, 7, 1, 4]);
        let expected_cups = as_vec_deque(&[2, 7, 1, 4]);
        let expected_taken = (6, 3, 5);

        let taken = take_cups(&mut cups);

        assert_eq!((cups, taken), (expected_cups, expected_taken));
    }

    #[test]
    fn test_place_cups() {
        let mut cups = as_vec_deque(&[6, 3, 1, 4]);
        let taken = (5, 2, 7);
        let destination = 1;
        let expected = as_vec_deque(&[6, 3, 1, 5, 2, 7, 4]);

        place_cups(&mut cups, taken, destination);

        assert_eq!(cups, expected);
    }

    #[test]
    fn test_find_next_cup_index() {
        let cups = as_vec_deque(&[6, 3, 1, 4]);

        assert_eq!(find_next_cup_index(&cups, 1), 3);
        assert_eq!(find_next_cup_index(&cups, 3), 2);
        assert_eq!(find_next_cup_index(&cups, 4), 0);
        assert_eq!(find_next_cup_index(&cups, 6), 1);
    }

    #[test]
    fn test_safe_decrement() {
        assert_eq!(safe_decrement(1, 1, 5), 5);
        assert_eq!(safe_decrement(3, 1, 5), 2);
        assert_eq!(safe_decrement(5, 1, 5), 4);
    }

    mod part1 {
        use super::*;

        const PART: Part = Part::One;

        #[test]
        fn test_play_10_moves() {
            assert_eq!(play(389125467, 10, 9, PART), 92658374);
        }

        #[test]
        fn test_play_100_moves() {
            assert_eq!(play(389125467, 100, 9, PART), 67384529);
        }
    }

    // FIXME: Too slow...
    // mod part2 {
    //     use super::*;
    //
    //     const PART: Part = Part::Two;
    //
    //     #[test]
    //     fn test_play() {
    //         assert_eq!(play(389125467, 10000000, 1000000, PART), 149245887792);
    //     }
    // }

    fn as_vec_deque(nrs: &[CupNumber]) -> VecDeque<CupNumber> {
        nrs.iter().cloned().collect()
    }
}
