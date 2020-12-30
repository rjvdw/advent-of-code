use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs::File;
use std::process::exit;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::input::WithReadLines;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let packages = File::open(&args[1]).read_lines(1).collect::<Vec<u32>>();

    if weight(&packages) % 12 != 0 {
        eprintln!("The total weight of the packages must be a multiple of 3 and of 4.");
        exit(1);
    }

    match balance_packages_without_trunk(&packages) {
        Some(qe_score) => println!(
            "Not considering the trunk, the quantum entanglement of the first group of packages is: {}.",
            qe_score
        ),
        None => eprintln!("There is no way to balance the packages."),
    }

    match balance_packages_with_trunk(&packages) {
        Some(qe_score) => println!(
            "Considering the trunk, the quantum entanglement of the first group of packages is: {}.",
            qe_score
        ),
        None => eprintln!("There is no way to balance the packages."),
    }
}

fn balance_packages_without_trunk(packages: &[u32]) -> Option<u64> {
    let weight_per_group = weight(&packages) / 3;
    let mut groupings = find_shortest_group(packages, weight_per_group);
    groupings.sort_unstable_by_key(|grouping| quantum_entanglement(grouping));

    for group1 in &groupings {
        let rest = subtract(packages, group1);
        // verify that the remaining packages can be split evenly across groups 2 and 3.
        if find_any_group(&rest, weight_per_group).is_some() {
            return Some(quantum_entanglement(group1));
        }
    }

    None
}

fn balance_packages_with_trunk(packages: &[u32]) -> Option<u64> {
    let weight_per_group = weight(&packages) / 4;
    let groupings = find_shortest_group(packages, weight_per_group);

    // FIXME: Just like part 1, verify that the remaining packages can actually be split in 3 groups
    // groupings.sort_unstable_by_key(|grouping| quantum_entanglement(grouping));
    // for group1 in &groupings {
    //     let rest = subtract(packages, group1);
    //     // verify that the remaining packages can be split evenly across groups 2, 3, and 4.
    //     if let Some(group2) = find_any_group(&rest, weight_per_group) {
    //         let rest = subtract(&rest, &group2);
    //         if let Some(_) = find_any_group(&rest, weight_per_group) {
    //             return Some(quantum_entanglement(group1));
    //         }
    //     }
    // }
    // None

    groupings.iter().map(|g| quantum_entanglement(g)).min()
}

fn find_any_group(packages: &[u32], desired_weight: u32) -> Option<Vec<u32>> {
    let mut groupings: HashSet<Vec<u32>> = HashSet::new();
    groupings.insert(Vec::new());

    for &package in packages {
        let mut next_groupings = HashSet::new();
        for mut grouping in groupings {
            let grouping_weight = weight(&grouping);
            next_groupings.insert(grouping.to_vec());
            match (grouping_weight + package).cmp(&desired_weight) {
                Ordering::Equal => {
                    grouping.push(package);
                    return Some(grouping);
                }
                Ordering::Less => {
                    grouping.push(package);
                    next_groupings.insert(grouping);
                }
                _ => {}
            }
        }
        groupings = next_groupings;
    }

    None
}

fn find_shortest_group(packages: &[u32], desired_weight: u32) -> Vec<Vec<u32>> {
    let mut groupings: HashSet<Vec<u32>> = HashSet::new();
    groupings.insert(Vec::new());
    let mut shortest = usize::MAX;

    for &package in packages {
        let mut next_groupings = HashSet::new();
        for mut grouping in groupings {
            let grouping_weight = weight(&grouping);
            if grouping_weight == desired_weight {
                if grouping.len() == shortest {
                    next_groupings.insert(grouping);
                }
            } else if grouping.len() < shortest {
                next_groupings.insert(grouping.to_vec());
                match (grouping_weight + package).cmp(&desired_weight) {
                    Ordering::Equal => {
                        grouping.push(package);
                        shortest = grouping.len();
                        next_groupings.insert(grouping);
                    }
                    Ordering::Less => {
                        grouping.push(package);
                        next_groupings.insert(grouping);
                    }
                    _ => {}
                }
            }
        }
        groupings = next_groupings;
    }

    groupings
        .iter()
        .filter(|g| g.len() == shortest && weight(g) == desired_weight)
        .cloned()
        .collect()
}

fn subtract(p1: &[u32], p2: &[u32]) -> Vec<u32> {
    let mut p1 = p1.to_vec();
    let mut p2 = p2.to_vec();
    while let Some(v) = p2.pop() {
        p1.remove(p1.iter().position(|x| *x == v).unwrap());
    }
    p1
}

fn weight(packages: &[u32]) -> u32 {
    packages.iter().sum()
}

fn quantum_entanglement(packages: &[u32]) -> u64 {
    packages.iter().map(|v| *v as u64).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_balance_packages_without_trunk() {
        let packages = vec![1, 2, 3, 4, 5, 7, 8, 9, 10, 11];
        assert_eq!(balance_packages_without_trunk(&packages), Some(99));
    }

    #[test]
    fn test_balance_packages_with_trunk() {
        let packages = vec![1, 2, 3, 4, 5, 7, 8, 9, 10, 11];
        assert_eq!(balance_packages_with_trunk(&packages), Some(44));
    }
}
