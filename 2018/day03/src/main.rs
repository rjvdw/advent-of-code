use std::collections::HashSet;
use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::input::WithReadLines;

use crate::claim::Claim;

mod claim;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let claims = File::open(&args[1]).read_lines(1).collect::<Vec<Claim>>();

    let (overlap, safe_squares) = find_overlap(&claims);
    println!(
        "There are {} square inches claimed by multiple elves.",
        overlap
    );
    println!(
        "Squares #{:?} do not overlap with any other square.",
        safe_squares
    );
}

fn find_overlap(claims: &[Claim]) -> (usize, Vec<usize>) {
    let mut claimed = HashSet::new();
    let mut overlap = HashSet::new();

    for claim in claims {
        for point in claim {
            if claimed.contains(&point) {
                overlap.insert(point);
            } else {
                claimed.insert(point);
            }
        }
    }

    let safe_squares = claims
        .iter()
        .filter(|claim| claim.into_iter().all(|p| !overlap.contains(&p)))
        .map(|claim| claim.get_nr())
        .collect();

    (overlap.len(), safe_squares)
}

#[cfg(test)]
mod tests {
    use rdcl_aoc_helpers::input::WithAsRecords;

    use super::*;

    #[test]
    fn test_find_overlap() {
        let claims = vec!["#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4", "#3 @ 5,5: 2x2"]
            .as_records::<Claim>()
            .unwrap();

        assert_eq!(find_overlap(&claims), (4, vec![3]));
    }
}
