use std::cmp::Ordering;

/// Checks how many elements are shared between two *sorted* lists.
pub fn count_overlap(one: &[i64], other: &[i64]) -> usize {
    let mut count = 0;
    let mut i = 0;
    let mut j = 0;

    while i < one.len() && j < other.len() {
        match one[i].cmp(&other[j]) {
            Ordering::Less => {
                i += 1;
            }
            Ordering::Equal => {
                count += 1;
                i += 1;
                j += 1;
            }
            Ordering::Greater => {
                j += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_overlap() {
        assert_eq!(count_overlap(&[1, 2, 3, 4, 5], &[2, 4, 5, 6]), 3);
        assert_eq!(count_overlap(&[2, 4, 5, 6], &[1, 2, 3, 4, 5]), 3);
    }
}
