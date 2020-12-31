use std::collections::HashSet;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;

use crate::item::Item;
use crate::parse::parse_input;
use crate::state::{Direction, State};

mod item;
mod parse;
mod state;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let floors = parse_input(&args[1]).or_exit_with(1);

    match find_quickest_solution(&floors) {
        Some(steps) => println!(
            "It takes {} steps to get everything to the fourth floor.",
            steps
        ),
        None => eprintln!("It is impossible to get everything to the fourth floor."),
    }

    let floors = add_additional_parts(&floors);

    match find_quickest_solution(&floors) {
        Some(steps) => println!(
            "After adding the additional parts, it takes {} steps to get everything to the fourth floor.",
            steps
        ),
        None => eprintln!("After adding the additional parts, it is impossible to get everything to the fourth floor."),
    }
}

fn add_additional_parts(floors: &[Vec<Item>]) -> Vec<Vec<Item>> {
    let mut new_floors = Vec::new();
    for (idx, floor) in floors.iter().enumerate() {
        if idx == 0 {
            let mut new_floor = floor.to_vec();
            new_floor.push(Item::Generator("elerium".to_string()));
            new_floor.push(Item::Microchip("elerium".to_string()));
            new_floor.push(Item::Generator("dilithium".to_string()));
            new_floor.push(Item::Microchip("dilithium".to_string()));
            new_floors.push(new_floor);
        } else {
            new_floors.push(floor.to_vec());
        }
    }
    new_floors
}

fn find_quickest_solution(floors: &[Vec<Item>]) -> Option<usize> {
    let mut start_floors = floors.to_vec();
    start_floors.iter_mut().for_each(|f| f.sort_unstable());
    let start_state = State {
        floors: start_floors,
        floor: 0,
    };

    let mut seen: HashSet<State> = HashSet::new();
    seen.insert(start_state.clone());

    let mut states: HashSet<State> = HashSet::new();
    states.insert(start_state);

    let mut steps = 0;

    while !states.is_empty() {
        steps += 1;
        // println!(
        //     "We are {} steps deep, and are evaluating {} states.",
        //     steps,
        //     states.len()
        // );
        let mut next_states: HashSet<State> = HashSet::new();
        for state in states {
            let mut possible_next_states: HashSet<(Direction, u8, State)> = HashSet::new();
            let mut has_2_up = false;
            let mut has_1_down = false;
            for item1 in &state.current_floor() {
                for direction in &[Direction::Up, Direction::Down] {
                    if let Some(state) = state.use_elevator(direction.clone(), &[item1.clone()]) {
                        if state.is_end_state() {
                            return Some(steps);
                        }
                        if !seen.contains(&state) {
                            seen.insert(state.clone());
                            possible_next_states.insert((direction.clone(), 1, state));
                            if direction.is_down() {
                                has_1_down = true;
                            }
                        }
                    }
                }

                for item2 in &state.current_floor() {
                    if item1 != item2 {
                        for direction in &[Direction::Up, Direction::Down] {
                            if let Some(state) = state
                                .use_elevator(direction.clone(), &[item1.clone(), item2.clone()])
                            {
                                if state.is_end_state() {
                                    return Some(steps);
                                }
                                if !seen.contains(&state) {
                                    seen.insert(state.clone());
                                    possible_next_states.insert((direction.clone(), 2, state));
                                    if direction.is_up() {
                                        has_2_up = true;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            for (direction, count, state) in possible_next_states {
                if direction.is_up() && count == 1 && has_2_up {
                    continue;
                }

                if direction.is_down() && count == 2 && has_1_down {
                    continue;
                }

                next_states.insert(state);
            }
        }
        states = next_states;
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::parse::parse_line;
    use crate::state::State;

    use super::*;

    #[test]
    fn test_is_floor_valid_1() {
        let line =
            "The first floor contains a foo-compatible microchip and a bar-compatible microchip.";
        let (floor, _) = parse_line(line).unwrap();

        assert!(State::is_floor_valid(&floor));
    }

    #[test]
    fn test_is_floor_valid_2() {
        let line = "The first floor contains a foo-compatible microchip and a foo generator.";
        let (floor, _) = parse_line(line).unwrap();

        assert!(State::is_floor_valid(&floor));
    }

    #[test]
    fn test_is_floor_valid_3() {
        let line = "The first floor contains a bar-compatible microchip and a foo generator.";
        let (floor, _) = parse_line(line).unwrap();

        assert!(!State::is_floor_valid(&floor));
    }

    #[test]
    fn test_is_floor_valid_4() {
        let line = "The first floor contains a bar generator and a foo generator.";
        let (floor, _) = parse_line(line).unwrap();

        assert!(State::is_floor_valid(&floor));
    }

    #[test]
    fn test_find_quickest_solution() {
        let floors = vec![
            "The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.",
            "The second floor contains a hydrogen generator.",
            "The third floor contains a lithium generator.",
            "The fourth floor contains nothing relevant.",
        ].iter().map(|s| parse_line(s).unwrap().0).collect::<Vec<Vec<Item>>>();

        assert_eq!(find_quickest_solution(&floors), Some(11));
    }
}
