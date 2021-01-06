use std::collections::HashMap;
use std::fs::File;
use std::mem;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;
use rdcl_aoc_helpers::input::WithReadMultiLines;

use forest::Forest;

fn main() {
    let args = get_args(&["<input file>", "<p1 steps>", "<p2 steps>"], 1);
    let mut forest = File::open(&args[1])
        .read_multi_lines::<Forest>(1)
        .next()
        .or_exit_with(1);
    let mut p1_steps = args[2].parse::<usize>().or_exit_with(1);
    let mut p2_steps = args[3].parse::<usize>().or_exit_with(1);

    if p1_steps > p2_steps {
        mem::swap(&mut p1_steps, &mut p2_steps);
    }

    let mut previous_state = String::new();
    let mut seen = HashMap::new();

    for _ in 1..=p1_steps {
        let state = forest.next_iteration();
        seen.insert(
            state.to_string(),
            (forest.get_resource_value(), String::new()),
        );
        seen.entry(previous_state)
            .and_modify(|v| v.1 = state.to_string());
        previous_state = state.to_string();
    }

    println!(
        "After {} minutes, the resource value is {}.",
        p1_steps,
        forest.get_resource_value()
    );

    for i in p1_steps + 1..=p2_steps {
        let state = forest.next_iteration();
        if seen.contains_key(&state) {
            // We have already seen this state. This means the state started looping. So if we just
            // follow this loop for (remaining steps) % (loop size), then we will find the desired
            // state.

            // close the loop
            seen.entry(previous_state)
                .and_modify(|v| v.1 = state.to_string());

            // find the loop size
            let mut loop_size = 1;
            let mut val = seen.get(&state).unwrap();
            while val.1 != state {
                loop_size += 1;
                val = seen.get(&val.1).unwrap();
            }

            // call forest.next() for the required number of times
            let mut remaining = (p2_steps - i) % loop_size;
            let mut val = seen.get(&state).unwrap();
            while remaining != 0 {
                forest.next_iteration();
                remaining -= 1;
                val = seen.get(&val.1).unwrap();
            }

            break;
        }
        seen.insert(
            state.to_string(),
            (forest.get_resource_value(), String::new()),
        );
        seen.entry(previous_state)
            .and_modify(|v| v.1 = state.to_string());
        previous_state = state.to_string();
    }

    println!(
        "After {} minutes, the resource value is {}.",
        p2_steps,
        forest.get_resource_value()
    );
}
