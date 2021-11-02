use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;
use rdcl_aoc_helpers::input::WithReadLines;
use rdcl_aoc_helpers::math::abs_diff;

/// The difference between the code point of an uppercase letter and its corresponding lower case
/// letter.
const LU_DIFF: u8 = 32;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let polymer = File::open(&args[1])
        .read_lines::<String>(1)
        .next()
        .or_exit_with(1);

    let reduced = reduce_polymer(&polymer);
    println!("The reduced polymer has length {}.", reduced.len());
    let (stripped_units, reduced) = find_shortest_reduction(&polymer);
    println!(
        "The shortest reduction is achieved by removing the units {}, and has length {}.",
        stripped_units,
        reduced.len()
    );
}

fn find_shortest_reduction(polymer: &str) -> (String, String) {
    let mut shortest = polymer.to_string();
    let mut stripped_unit = 0;
    for unit in b'A'..=b'Z' {
        let stripped = polymer
            .replace(unit as char, "")
            .replace((unit + LU_DIFF) as char, "");
        if stripped.len() < polymer.len() {
            let reduced = reduce_polymer(&stripped);
            if reduced.len() < shortest.len() {
                shortest = reduced;
                stripped_unit = unit;
            }
        }
    }

    let mut stripped_units = String::new();
    stripped_units.push(stripped_unit as char);
    stripped_units.push((stripped_unit + LU_DIFF) as char);
    (stripped_units, shortest)
}

fn reduce_polymer(polymer: &str) -> String {
    let units = polymer.as_bytes();

    let mut ll = vec![0; units.len()];
    let mut ll_rev = vec![units.len() - 1; units.len()];
    for (idx, _) in units.iter().skip(1).enumerate() {
        ll[idx] = idx + 1;
        ll_rev[idx + 1] = idx;
    }

    let mut current = 0;
    while current < ll[current] {
        let next = ll[current];
        if abs_diff(units[current], units[next]) == LU_DIFF {
            let prev = ll_rev[current];
            ll[prev] = ll[next]; // removes current and next from ll
            ll_rev[ll[next]] = prev; // removes current and next from ll_rev
            if prev < current {
                current = prev;
            } else {
                // in this case we just removed units from the start
                current = ll[next];
            }
        } else {
            current = next;
        }
    }

    current = ll[current];
    reconstruct(units, &ll, current)
}

fn reconstruct(units: &[u8], ll: &[usize], mut current: usize) -> String {
    let mut reduced = String::new();
    while current < ll[current] {
        reduced.push(units[current] as char);
        current = ll[current];
    }
    reduced.push(units[current] as char);

    reduced
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reduce_polymer() {
        assert_eq!(reduce_polymer("dabAcCaCBAcCcaDA"), "dabCBAcaDA");
    }

    #[test]
    fn test_find_shortest_reduction() {
        assert_eq!(
            find_shortest_reduction("dabAcCaCBAcCcaDA"),
            ("Cc".to_string(), "daDA".to_string())
        );
    }
}
