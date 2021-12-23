extern crate rdcl_aoc_helpers;

use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::{ParseError, WithOrExit};

use crate::amphipod::Amphipod;
use crate::amphipod_locations::{MutateAmphipodLocations, SearchAmphipodLocations};
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
    let (amphipods, burrow_depth) = parse_input(lines).or_exit_with(1);

    match find_cheapest_path(&amphipods, burrow_depth) {
        Some(cost) => println!("The cheapest solution has cost {}", cost),
        None => eprintln!("No solution exists."),
    }
}

fn find_cheapest_path(amphipods: &[(Node, Amphipod)], burrow_depth: usize) -> Option<usize> {
    // keep track of the candidates that we still need to explore
    let mut candidates = BinaryHeap::<Candidate>::new();
    let mut costs = HashMap::<Candidate, usize>::new();

    // the starting situation
    let mut initial_candidate = Candidate {
        amphipods: amphipods.to_vec(),
        exhausted: vec![false; amphipods.len()],
        burrow_depth,
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

        if check_move_to_burrow(&candidate, &mut candidates, &mut costs, &mut cheapest) {
            continue;
        }

        check_move_to_hallway(&candidate, &mut candidates, &mut costs, &mut cheapest);
    }

    cheapest
}

/// if any of the amphipods can move directly to their burrow, then that's the only logical step to
/// take this iteration
fn check_move_to_burrow(
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

        if let Some(target) = candidate.move_to_burrow(node, amphipod) {
            let cost_so_far = cost_so_far + amphipod.compute_energy(node.distance_to(&target));

            if let Some(cheapest_so_far) = cheapest {
                if cost_so_far >= *cheapest_so_far {
                    // the candidate cannot possible lead to a cheaper solution than the one we already have
                    return true;
                }
            }

            let mut candidate = candidate.clone();
            candidate.amphipods.move_amphipod(amphipod, &target);
            candidate.exhausted[amphipod.index()] = true;
            candidate.normalize();

            if candidate.is_done() {
                *cheapest = match cheapest {
                    None => Some(cost_so_far),
                    Some(v) if *v > cost_so_far => Some(cost_so_far),
                    _ => *cheapest,
                };
            } else if let Some(existing_cost) = costs.get_mut(&candidate) {
                if cost_so_far < *existing_cost {
                    candidates.push(candidate);
                    *existing_cost = cost_so_far;
                }
            } else {
                // if we did not already encounter this state, push it to candidates
                candidates.push(candidate.clone());
                costs.insert(candidate, cost_so_far);
            }

            return true;
        }
    }
    false
}

/// if no amphipods can move directly to their burrow, start moving amphipods to the hallway
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

        for neighbour in candidate.move_to_hallway(node, amphipod) {
            let cost_so_far = cost_so_far + amphipod.compute_energy(node.distance_to(&neighbour));

            if let Some(cheapest_so_far) = cheapest {
                if cost_so_far >= *cheapest_so_far {
                    // this neighbour cannot be cheaper than the best we found so far
                    continue;
                }
            }

            let mut next_candidate = candidate.clone();
            next_candidate.amphipods.move_amphipod(amphipod, &neighbour);
            next_candidate.normalize();

            if next_candidate.is_done() {
                // all candidates are home
                *cheapest = Some(cost_so_far);
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
    }
}

fn parse_input<I>(input: I) -> Result<(Vec<(Node, Amphipod)>, usize), ParseError>
where
    I: Iterator<Item = io::Result<String>>,
{
    let mut amphipods = Vec::new();
    let mut burrow_min = 0;
    let mut burrow_max = 0;
    for (y, line) in input.enumerate() {
        for (x, ch) in line?.chars().enumerate() {
            match ch {
                ' ' | '.' | '#' => {}
                _ => {
                    if burrow_min == 0 {
                        burrow_min = y;
                    }
                    burrow_max = y;
                    amphipods.push((Node { x, y }, Amphipod::new(ch, amphipods.len())));
                }
            }
        }
    }
    Ok((amphipods, burrow_max - burrow_min + 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let (amphipods, burrow_depth) = small_input();
        assert_eq!(burrow_depth, 2);
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
        let (amphipods, burrow_depth) = large_input();
        assert_eq!(find_cheapest_path(&amphipods, burrow_depth), Some(44169));
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
