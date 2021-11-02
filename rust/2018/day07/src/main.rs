use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::{ParseError, WithOrExit};

fn main() {
    let args = get_args(&["<input file>", "<nr workers>", "<step time>"], 1);
    let (instructions, steps) = parse_input(&args[1]).or_exit_with(1);
    let nr_workers = args[2].parse::<usize>().or_exit_with(1);
    let step_time = args[3].parse::<usize>().or_exit_with(1);

    println!(
        "The correct build order is {}.",
        find_correct_order(&instructions, &steps)
    );

    println!(
        "It will take {} seconds to build the sleigh.",
        determine_build_time(&instructions, &steps, nr_workers, step_time)
    );
}

fn find_correct_order(instructions: &[(char, char)], steps: &[char]) -> String {
    let mut steps = steps.to_vec();
    steps.sort_unstable();
    let mut processed: HashSet<char> = HashSet::new();
    let mut sorted = Vec::new();

    while processed.len() < steps.len() {
        for &step in &steps {
            let ready = || {
                !instructions
                    .iter()
                    .any(|(left, right)| !processed.contains(left) && step == *right)
            };

            if !processed.contains(&step) && ready() {
                sorted.push(step);
                processed.insert(step);
                break;
            }
        }
    }

    let mut correct = String::new();
    for ch in sorted {
        correct.push(ch);
    }
    correct
}

fn determine_build_time(
    instructions: &[(char, char)],
    steps: &[char],
    nr_workers: usize,
    step_time: usize,
) -> usize {
    let mut timer = 0;
    let mut steps = steps.to_vec();
    steps.sort_unstable();
    let mut processed: HashSet<char> = HashSet::new();
    let mut processing: HashMap<char, usize> = HashMap::new();

    while processed.len() < steps.len() {
        // Check if any of the steps that are in progress have finished.
        for (step, end_time) in &processing {
            if timer >= *end_time {
                processed.insert(*step);
            }
        }

        // All steps that are finished can be removed from `processing`.
        for step in &processed {
            processing.remove(step);
        }

        // Keep track of whether any new steps were started. Otherwise, we can increase the clock.
        let mut new_steps_started = false;

        // Check if there are still any workers available.
        if processing.len() < nr_workers {
            // Search for the first step that can be executed.
            for &step in &steps {
                // Check if there are any rules preventing this step from being executed.
                let ready = || {
                    !instructions
                        .iter()
                        .any(|(left, right)| !processed.contains(left) && step == *right)
                };

                // If this step is not already in progress or processed...
                if processing.len() < nr_workers
                    && !processing.contains_key(&step)
                    && !processed.contains(&step)
                    && ready()
                {
                    // Determine when the step will finish.
                    let end_time = timer + step_time + ((step as u8) - b'A' + 1) as usize;
                    processing.insert(step, end_time);
                    new_steps_started = true;
                }
            }
        }

        if !new_steps_started {
            timer = processing.values().min().copied().unwrap_or(timer);
        }
    }

    timer
}

#[allow(clippy::type_complexity)]
fn parse_input(path: &str) -> Result<(Vec<(char, char)>, Vec<char>), ParseError> {
    let mut instructions = Vec::new();
    let mut steps = HashSet::new();
    let file = File::open(path)?;
    for line in BufReader::new(file).lines() {
        let line = line?;
        let left = line[5..6].parse::<char>()?;
        let right = line[36..37].parse::<char>()?;
        instructions.push((left, right));
        steps.insert(left);
        steps.insert(right);
    }
    Ok((instructions, steps.iter().copied().collect()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_correct_order() {
        let instructions = vec![
            ('C', 'A'),
            ('C', 'F'),
            ('A', 'B'),
            ('A', 'D'),
            ('B', 'E'),
            ('D', 'E'),
            ('F', 'E'),
        ];
        let steps = vec!['A', 'B', 'C', 'D', 'E', 'F'];

        assert_eq!(find_correct_order(&instructions, &steps), "CABDFE");
    }

    #[test]
    fn test_determine_build_time() {
        let instructions = vec![
            ('C', 'A'),
            ('C', 'F'),
            ('A', 'B'),
            ('A', 'D'),
            ('B', 'E'),
            ('D', 'E'),
            ('F', 'E'),
        ];
        let steps = vec!['A', 'B', 'C', 'D', 'E', 'F'];

        assert_eq!(determine_build_time(&instructions, &steps, 2, 0), 15);
    }
}
