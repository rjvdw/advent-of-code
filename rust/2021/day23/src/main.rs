extern crate rdcl_aoc_helpers;

use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::{ParseError, WithOrExit};

use crate::amphipod::Amphipod;
use crate::amphipod_locations::SearchAmphipodLocations;
use crate::candidate::Candidate;
use crate::node::Node;

mod amphipod;
mod amphipod_locations;
mod candidate;
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

fn find_cheapest_path(amphipods: &[(Node, Amphipod)], side_room_depth: usize) -> Option<usize> {
    // keep track of the candidates that we still need to explore
    let mut candidates = BinaryHeap::<Candidate>::new();
    let mut costs = HashMap::<Candidate, usize>::new();

    // the starting situation
    let mut initial_candidate = Candidate {
        amphipods: amphipods.to_vec(),
        exhausted: vec![false; amphipods.len()],
        side_room_depth,
    };
    initial_candidate.normalize();
    candidates.push(initial_candidate.clone());
    costs.insert(initial_candidate, 0);

    // keep track of the cheapest solution we found so far
    let mut cheapest = None;

    // as long as there still candidates...
    while let Some(candidate) = candidates.pop() {
        if let Some(cheapest_so_far) = &cheapest {
            if costs.get(&candidate).unwrap() >= cheapest_so_far {
                // this candidate cannot be cheaper than the best we found so far
                continue;
            }
        }

        if check_move_to_side_room(&candidate, &mut candidates, &mut costs, &mut cheapest) {
            continue;
        }

        check_move_to_hallway(&candidate, &mut candidates, &mut costs, &mut cheapest);
    }

    cheapest
}

/// if any of the amphipods can move directly to their side room, then that's the only logical step
/// to take this iteration
fn check_move_to_side_room(
    candidate: &Candidate,
    candidates: &mut BinaryHeap<Candidate>,
    costs: &mut HashMap<Candidate, usize>,
    cheapest: &mut Option<usize>,
) -> bool {
    let cost_so_far = *costs.get(candidate).unwrap();

    for (node, amphipod) in &candidate.amphipods {
        // this amphipod is exhausted and can no longer move
        if candidate.exhausted[amphipod.index()] {
            continue;
        }

        // the path out is blocked, so no point in checking this node
        if candidate.exit_is_blocked(node) {
            continue;
        }

        if let Some(target) = candidate.find_move_to_side_room(node, amphipod) {
            let cost_so_far = cost_so_far + amphipod.compute_energy(node.distance_to(&target));

            if let Some(cheapest_so_far) = cheapest {
                if cost_so_far >= *cheapest_so_far {
                    // the candidate cannot possible lead to a cheaper solution than the one we already have
                    return true;
                }
            }

            let mut next_candidate = candidate.clone();
            next_candidate.amphipods[amphipod.index()].0 = target;
            next_candidate.exhausted[amphipod.index()] = true;

            check_candidate(next_candidate, cost_so_far, candidates, costs, cheapest);

            return true;
        }
    }
    false
}

/// if no amphipods can move directly to their side room , start moving amphipods to the hallway
fn check_move_to_hallway(
    candidate: &Candidate,
    candidates: &mut BinaryHeap<Candidate>,
    costs: &mut HashMap<Candidate, usize>,
    cheapest: &mut Option<usize>,
) {
    let cost_so_far = *costs.get(candidate).unwrap();

    for (node, amphipod) in &candidate.amphipods {
        // this amphipod is exhausted and can no longer move
        if candidate.exhausted[amphipod.index()] {
            continue;
        }

        // the path out is blocked, so no point in checking this node
        if candidate.exit_is_blocked(node) {
            continue;
        }

        for neighbour in candidate.find_moves_to_hallway(node, amphipod) {
            let cost_so_far = cost_so_far + amphipod.compute_energy(node.distance_to(&neighbour));

            if let Some(cheapest_so_far) = cheapest {
                if cost_so_far >= *cheapest_so_far {
                    // this neighbour cannot be cheaper than the best we found so far
                    continue;
                }
            }

            let mut next_candidate = candidate.clone();
            next_candidate.amphipods[amphipod.index()].0 = neighbour;

            check_candidate(next_candidate, cost_so_far, candidates, costs, cheapest);
        }
    }
}

fn check_candidate(
    mut next_candidate: Candidate,
    cost_so_far: usize,
    candidates: &mut BinaryHeap<Candidate>,
    costs: &mut HashMap<Candidate, usize>,
    cheapest: &mut Option<usize>,
) {
    next_candidate.normalize();

    if next_candidate.is_done() {
        *cheapest = match cheapest {
            None => Some(cost_so_far),
            Some(v) if *v > cost_so_far => Some(cost_so_far),
            _ => *cheapest,
        };
    } else if let Some(existing_cost) = costs.get_mut(&next_candidate) {
        if cost_so_far < *existing_cost {
            candidates.push(next_candidate);
            *existing_cost = cost_so_far;
        }
    } else {
        // if we did not already encounter this state, push it to candidates
        candidates.push(next_candidate.clone());
        costs.insert(next_candidate, cost_so_far);
    }
}

fn parse_input<I>(input: I) -> Result<(Vec<(Node, Amphipod)>, usize), ParseError>
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
                    amphipods.push((Node { x, y }, Amphipod::new(ch, amphipods.len())));
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
                (Node { y: 2, x: 3 }, Amphipod::new('B', 0)),
                (Node { y: 2, x: 5 }, Amphipod::new('C', 1)),
                (Node { y: 2, x: 7 }, Amphipod::new('B', 2)),
                (Node { y: 2, x: 9 }, Amphipod::new('D', 3)),
                (Node { y: 3, x: 3 }, Amphipod::new('A', 4)),
                (Node { y: 3, x: 5 }, Amphipod::new('D', 5)),
                (Node { y: 3, x: 7 }, Amphipod::new('C', 6)),
                (Node { y: 3, x: 9 }, Amphipod::new('A', 7)),
            ]
        );
    }

    #[test]
    fn test_find_cheapest_path() {
        let (amphipods, side_room_depth) = large_input();
        assert_eq!(find_cheapest_path(&amphipods, side_room_depth), Some(44169));
    }

    fn small_input() -> (Vec<(Node, Amphipod)>, usize) {
        let input = vec![
            Ok("#############".to_string()),
            Ok("#...........#".to_string()),
            Ok("###B#C#B#D###".to_string()),
            Ok("  #A#D#C#A#".to_string()),
            Ok("  #########".to_string()),
        ];

        parse_input(input.into_iter()).unwrap()
    }

    fn large_input() -> (Vec<(Node, Amphipod)>, usize) {
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
