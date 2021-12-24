extern crate rdcl_aoc_helpers;

use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::{ParseError, WithOrExit};

use crate::amphipod::Amphipod;
use crate::node::Node;
use crate::state::State;

mod amphipod;
mod node;
mod state;

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

    let mut initial_state = State {
        amphipods: amphipods.to_vec(),
        side_room_depth,
    };
    initial_state.normalize();
    queue.push(initial_state.clone());
    costs.insert(initial_state, 0);

    let mut cheapest = None;

    while let Some(state) = queue.pop() {
        if let Some(cheapest_so_far) = &cheapest {
            if costs.get(&state).unwrap() >= cheapest_so_far {
                continue;
            }
        }

        if let Some((next_state, cost)) = state.find_move_to_side_room() {
            process_next_state(next_state, cost, &mut queue, &mut costs, &mut cheapest);
        } else {
            for (next_state, cost) in state.find_moves_to_hallway() {
                process_next_state(next_state, cost, &mut queue, &mut costs, &mut cheapest);
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
    mut next_state: State,
    cost: usize,
    queue: &mut BinaryHeap<State>,
    costs: &mut HashMap<State, usize>,
    cheapest: &mut Option<usize>,
) {
    next_state.normalize();

    if next_state.finished() {
        *cheapest = match cheapest {
            None => Some(cost),
            Some(v) if *v > cost => Some(cost),
            _ => *cheapest,
        };
    } else if let Some(existing) = costs.get_mut(&next_state) {
        if cost < *existing {
            queue.push(next_state);
            *existing = cost;
        }
    } else {
        queue.push(next_state.clone());
        costs.insert(next_state, cost);
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
