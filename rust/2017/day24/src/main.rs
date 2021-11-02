use std::fs::File;
use std::io::{BufRead, BufReader, Read};

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::{ParseError, WithOrExit};

#[derive(Debug, Clone, Eq, PartialEq)]
struct Score {
    strength: u32,
    length: u32,
}

type Component = (u32, u32);

fn main() {
    let args = get_args(&["<input file>"], 1);
    let file = File::open(&args[1]).or_exit_with(1);
    let components = parse_input(file).or_exit_with(1);

    println!(
        "The strongest bridge that can be built out of these components has strength {}.",
        build_strongest_bridge(&components).strength
    );

    println!(
        "The strongest bridge that can be built out of these components has strength {}.",
        build_longest_bridge(&components).strength
    );
}

fn build_strongest_bridge(components: &[Component]) -> Score {
    build_best_bridge(components, 0, &|current, next| {
        next.strength > current.strength
    })
}

fn build_longest_bridge(components: &[Component]) -> Score {
    build_best_bridge(components, 0, &|current, next| {
        next.length > current.length
            || (next.length == current.length && next.strength > current.strength)
    })
}

fn build_best_bridge<P>(components: &[Component], end: u32, is_better: &P) -> Score
where
    P: Fn(&Score, &Score) -> bool,
{
    let mut best_score_so_far = Score {
        strength: end,
        length: 0,
    };

    for item in find_candidate_components(components, end) {
        let (candidate, remaining) = item;

        let mut build_sub_bridge = |port1: u32, port2: u32| {
            let mut next_score = build_best_bridge(&remaining, port2, is_better);

            next_score.strength += end + port1;
            next_score.length += 1;

            if is_better(&best_score_so_far, &next_score) {
                best_score_so_far = next_score;
            }
        };

        if candidate.0 == end {
            build_sub_bridge(candidate.0, candidate.1);
        }
        if candidate.0 != candidate.1 && candidate.1 == end {
            build_sub_bridge(candidate.1, candidate.0);
        }
    }

    best_score_so_far
}

fn find_candidate_components(
    components: &[Component],
    end: u32,
) -> Vec<(Component, Vec<Component>)> {
    components
        .iter()
        .filter(|&&(l, r)| l == end || r == end)
        .map(|&candidate| {
            let mut remaining: Vec<Component> = Vec::with_capacity(components.len() - 1);
            let mut found_it = false;
            for &(left, right) in components {
                if !found_it && (left, right) == candidate {
                    found_it = true;
                } else {
                    remaining.push((left, right));
                }
            }
            (candidate, remaining)
        })
        .collect()
}

fn parse_input<R: Read>(readable: R) -> Result<Vec<Component>, ParseError> {
    let mut components = vec![];
    for line in BufReader::new(readable).lines() {
        let line = line?;
        let ports: Vec<&str> = line.split('/').collect();
        components.push((ports[0].parse()?, ports[1].parse()?));
    }
    Ok(components)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_strongest_bridge() {
        let components = vec![
            (0, 2),
            (2, 2),
            (2, 3),
            (3, 4),
            (3, 5),
            (0, 1),
            (10, 1),
            (9, 10),
        ];

        assert_eq!(
            build_strongest_bridge(&components),
            Score {
                strength: 31,
                length: 3
            }
        );
    }

    #[test]
    fn test_build_longest_bridge() {
        let components = vec![
            (0, 2),
            (2, 2),
            (2, 3),
            (3, 4),
            (3, 5),
            (0, 1),
            (10, 1),
            (9, 10),
        ];

        assert_eq!(
            build_longest_bridge(&components),
            Score {
                strength: 19,
                length: 4
            }
        );
    }
}
