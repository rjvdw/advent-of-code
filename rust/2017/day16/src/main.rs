use std::collections::{HashMap, VecDeque};
use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;

use crate::dance_move::{parse_input, DanceMove};

mod dance_move;

fn main() {
    let args = get_args(&["<input file>", "<nr programs>", "<iterations>"], 1);
    let file = File::open(&args[1]).or_exit_with(1);
    let dance_moves = parse_input(file).or_exit_with(1);
    let nr_programs = args[2].parse::<u8>().or_exit_with(1);
    let iterations = args[3].replace('_', "").parse::<u64>().or_exit_with(1);

    println!(
        "After the dance completes, the programs stand in order: '{}'.",
        dance(nr_programs, &dance_moves)
    );

    println!(
        "If the dance repeats {} times, the programs will stand in order: '{}'.",
        iterations,
        repeat_dance(nr_programs, &dance_moves, iterations)
    )
}

fn dance(nr_programs: u8, dance_moves: &[DanceMove]) -> String {
    repeat_dance(nr_programs, dance_moves, 1)
}

fn repeat_dance(nr_programs: u8, dance_moves: &[DanceMove], iterations: u64) -> String {
    let mut programs = VecDeque::with_capacity(nr_programs as usize);
    for i in 0..nr_programs {
        programs.push_back((i + b'a') as char);
    }

    let mut previous_order = as_string(&programs);
    let mut seen: HashMap<String, (u64, String)> = HashMap::new();

    for iteration in 0..iterations {
        for dance_move in dance_moves {
            dance_move.execute(&mut programs);
        }

        let mut order = as_string(&programs);

        seen.entry(previous_order.to_string())
            .and_modify(|v| v.1 = order.to_string());

        if let Some((previous_iteration, next_step)) = seen.get(&order) {
            let remaining_steps = (iterations - 1 - iteration) % (iteration - previous_iteration);
            let mut next_step = next_step.to_string();
            for _ in 0..remaining_steps {
                order = next_step.to_string();
                next_step = seen.get(&order).unwrap().1.to_string();
            }
            return order;
        } else {
            seen.insert(order.to_string(), (iteration, "todo".to_string()));
            previous_order = order;
        }
    }

    as_string(&programs)
}

fn as_string(programs: &VecDeque<char>) -> String {
    let mut order = String::new();
    for &program in programs {
        order.push(program);
    }
    order
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dance() {
        let dance_moves = vec![
            DanceMove::Spin(1),
            DanceMove::Exchange(3, 4),
            DanceMove::Partner('e', 'b'),
        ];

        assert_eq!(dance(5, &dance_moves), "baedc");
    }

    #[test]
    fn test_repeat_dance() {
        let dance_moves = vec![
            DanceMove::Spin(1),
            DanceMove::Exchange(3, 4),
            DanceMove::Partner('e', 'b'),
        ];

        assert_eq!(repeat_dance(5, &dance_moves, 2), "ceadb");
    }
}
