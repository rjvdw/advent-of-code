extern crate rdcl_aoc_helpers;

use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::{ParseError, WithOrExit};

use crate::amphipod::Amphipod;
use crate::burrow::Burrow;
use crate::node::Node;

mod amphipod;
mod burrow;
mod node;

fn main() {
    let args = get_args(&["<input file>"], 1);

    let file = File::open(&args[1]).or_exit_with(1);
    let lines = BufReader::new(file).lines();
    let (amphipods, side_room_depth) = parse_input(lines).or_exit_with(1);

    match find_cheapest_path(&amphipods, side_room_depth) {
        Some(cost) => println!("The cheapest solution has cost {}", cost),
        None => eprintln!("No solution exists."),
    }
}

fn find_cheapest_path(amphipods: &[Amphipod], side_room_depth: usize) -> Option<usize> {
    let mut queue = BinaryHeap::new();
    let mut costs = HashMap::new();

    let mut initial_state = Burrow {
        amphipods: amphipods.to_vec(),
        side_room_depth,
    };
    initial_state.normalize();
    queue.push(initial_state.clone());
    costs.insert(initial_state, 0);

    let mut cheapest = None;
    while let Some(burrow) = queue.pop() {
        let cost_so_far = *costs.get(&burrow).unwrap();
        if let Some(cheapest_so_far) = &cheapest {
            if cost_so_far >= *cheapest_so_far {
                // a cheaper solution has been found in the mean time
                continue;
            }
        }

        if let Some((next_state, cost)) = burrow.find_move_to_side_room() {
            process_next_state(
                next_state,
                cost_so_far + cost,
                &mut queue,
                &mut costs,
                &mut cheapest,
            );
        } else {
            for (next_state, cost) in burrow.find_moves_to_hallway() {
                process_next_state(
                    next_state,
                    cost_so_far + cost,
                    &mut queue,
                    &mut costs,
                    &mut cheapest,
                );
            }
        }
    }
    cheapest
}

/// Processes the next state:
/// * Normalize
/// * If all amphopids are home, update cheapest
/// * If we have already seen this state, check if we found a cheaper path to this state
/// * Add the state to the queue if needed
fn process_next_state(
    mut next_state: Burrow,
    cost: usize,
    queue: &mut BinaryHeap<Burrow>,
    costs: &mut HashMap<Burrow, usize>,
    cheapest: &mut Option<usize>,
) {
    if let Some(v) = cheapest {
        if cost >= *v {
            // this state is more expensive than the cheapest state we found
            return;
        }
    }

    if next_state.finished() {
        *cheapest = Some(cost);
    } else {
        next_state.normalize();
        if let Some(existing) = costs.get_mut(&next_state) {
            if cost < *existing {
                queue.push(next_state);
                *existing = cost;
            }
        } else {
            queue.push(next_state.clone());
            costs.insert(next_state, cost);
        }
    }
}

/// Checks where the amphipods currently are, given the puzzle input.
fn parse_input<I>(input: I) -> Result<(Vec<Amphipod>, usize), ParseError>
where
    I: Iterator<Item = io::Result<String>>,
{
    let mut amphipods = Vec::new();
    let mut min_depth = 0;
    let mut max_depth = 0;
    for (y, line) in input.enumerate() {
        for (x, ch) in line?.chars().enumerate() {
            match ch {
                ' ' | '.' | '#' => {}
                _ => {
                    if min_depth == 0 {
                        min_depth = y;
                    }
                    max_depth = y;
                    amphipods.push(Amphipod::new(ch, Node { y, x }));
                }
            }
        }
    }
    Ok((amphipods, max_depth - min_depth + 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let (amphipods, side_room_depth) = small_input();
        assert_eq!(side_room_depth, 2);
        assert_eq!(
            amphipods,
            vec![
                Amphipod::new('B', Node { y: 2, x: 3 }),
                Amphipod::new('C', Node { y: 2, x: 5 }),
                Amphipod::new('B', Node { y: 2, x: 7 }),
                Amphipod::new('D', Node { y: 2, x: 9 }),
                Amphipod::new('A', Node { y: 3, x: 3 }),
                Amphipod::new('D', Node { y: 3, x: 5 }),
                Amphipod::new('C', Node { y: 3, x: 7 }),
                Amphipod::new('A', Node { y: 3, x: 9 }),
            ]
        );
    }

    #[test]
    fn test_find_cheapest_path() {
        let (amphipods, side_room_depth) = small_input();
        assert_eq!(find_cheapest_path(&amphipods, side_room_depth), Some(12521));

        let (amphipods, side_room_depth) = large_input();
        assert_eq!(find_cheapest_path(&amphipods, side_room_depth), Some(44169));
    }

    fn small_input() -> (Vec<Amphipod>, usize) {
        let input = vec![
            Ok("#############".to_string()),
            Ok("#...........#".to_string()),
            Ok("###B#C#B#D###".to_string()),
            Ok("  #A#D#C#A#".to_string()),
            Ok("  #########".to_string()),
        ];

        parse_input(input.into_iter()).unwrap()
    }

    fn large_input() -> (Vec<Amphipod>, usize) {
        let input = vec![
            Ok("#############".to_string()),
            Ok("#...........#".to_string()),
            Ok("###B#C#B#D###".to_string()),
            Ok("  #D#C#B#A#".to_string()),
            Ok("  #D#B#A#C#".to_string()),
            Ok("  #A#D#C#A#".to_string()),
            Ok("  #########".to_string()),
        ];

        parse_input(input.into_iter()).unwrap()
    }
}
