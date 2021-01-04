use std::mem;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;

use crate::parse::{parse_input, RULE_SIZE};

mod parse;

fn main() {
    let args = get_args(&["<input file>", "<steps part 1>", "<steps part 2>"], 1);
    let (initial_state, rules) = parse_input(&args[1]).or_exit_with(1);
    let mut steps_part_1 = args[2].replace('_', "").parse::<usize>().or_exit_with(1);
    let mut steps_part_2 = args[3].replace('_', "").parse::<usize>().or_exit_with(1);

    if steps_part_2 < steps_part_1 {
        mem::swap(&mut steps_part_1, &mut steps_part_2);
    }

    let mut previous = (state_as_string(&initial_state), 0);
    let mut state = (initial_state, 0);

    for step in 1..=steps_part_2 {
        state = next(&state.0, state.1, rules);

        if step == steps_part_1 || step == steps_part_2 {
            print_solution(&state.0, step, state.1);
        }

        let s = state_as_string(&state.0);
        if s == previous.0 {
            println!("The pattern starts repeating at step {}.", step);
            let mut step = step;
            let diff = state.1 - previous.1;
            apply_repetition(&state.0, &mut step, &mut state.1, diff, steps_part_1);
            apply_repetition(&state.0, &mut step, &mut state.1, diff, steps_part_2);
            break;
        }
        previous = (s, state.1);
    }
}

fn print_solution(state: &[bool], step: usize, offset: i64) {
    println!(
        "After {} steps, the sum is {}.",
        step,
        calculate_sum(&state, offset)
    );
}

fn apply_repetition(
    state: &[bool],
    step: &mut usize,
    offset: &mut i64,
    diff: i64,
    desired_steps: usize,
) {
    if *step < desired_steps {
        *offset += ((desired_steps - *step) as i64) * diff;
        *step = desired_steps;
        print_solution(state, *step, *offset);
    }
}

fn state_as_string(state: &[bool]) -> String {
    let mut s = String::new();
    for &b in state {
        if b {
            s.push('1');
        } else {
            s.push('0');
        }
    }
    s
}

fn calculate_sum(state: &[bool], offset: i64) -> i64 {
    state
        .iter()
        .enumerate()
        .filter(|(_, &v)| v)
        .map(|(i, _)| i as i64 + offset)
        .sum()
}

fn next(state: &[bool], offset: i64, rules: u32) -> (Vec<bool>, i64) {
    let mut counts = vec![0; state.len() + RULE_SIZE - 1];
    for (i, _) in state.iter().enumerate().filter(|(_, &x)| x) {
        for j in 0..RULE_SIZE {
            counts[i + j] += 2u32.pow(j as u32);
        }
    }

    let mut next_state = vec![false; counts.len()];
    let mut first = usize::MAX;
    let mut is_first = true;
    let mut last_idx = 0;
    for (i, &count) in counts.iter().enumerate() {
        if (rules >> count) % 2 == 1 {
            if is_first {
                first = i;
                is_first = false;
            }
            next_state[i - first] = true;
            last_idx = i - first;
        }
    }
    next_state.truncate(last_idx + 1);

    (
        next_state,
        offset + (first as i64) - ((RULE_SIZE / 2) as i64),
    )
}

#[cfg(test)]
mod tests {
    use crate::parse::parse_test_input;

    use super::*;

    #[test]
    fn test_next() {
        let (initial_state, rules) = parse_test_input(&vec![
            "initial state: #..#.#..##......###...###",
            "",
            "...## => #",
            "..#.. => #",
            ".#... => #",
            ".#.#. => #",
            ".#.## => #",
            ".##.. => #",
            ".#### => #",
            "#.#.# => #",
            "#.### => #",
            "##.#. => #",
            "##.## => #",
            "###.. => #",
            "###.# => #",
            "####. => #",
        ]);

        print_state(&initial_state, 0);
        assert_eq!(state_len(&initial_state), 11);
        let mut state = next(&initial_state, 0, rules);
        print_state(&state.0, state.1);
        assert_eq!(state_len(&state.0), 7);
        state = next(&state.0, state.1, rules);
        print_state(&state.0, state.1);
        assert_eq!(state_len(&state.0), 11);
        for _ in 2..20 {
            state = next(&state.0, state.1, rules);
        }
        print_state(&state.0, state.1);
        assert_eq!(state_len(&state.0), 19);
        assert_eq!(calculate_sum(&state.0, state.1), 325);
    }

    fn state_len(state: &[bool]) -> usize {
        state.iter().filter(|&v| *v).count()
    }

    fn print_state(state: &[bool], offset: i64) {
        print!("[");
        for &s in state {
            if s {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("] (offset={})", offset);
    }
}
