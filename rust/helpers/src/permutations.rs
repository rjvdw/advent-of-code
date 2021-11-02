//! Produce all permutations of a given collection.

use std::iter::FromIterator;

/// Using [Heap's algorithm](https://en.wikipedia.org/wiki/Heap%27s_algorith), produce all
/// permutations of a given collection.
pub struct Permutations<T> {
    items: Vec<T>,
    size: usize,
    stack: Vec<usize>,
    index: usize,
    started: bool,
}

impl<T> Permutations<T> {
    /// Create a new instance from a Vec.
    pub fn new(items: Vec<T>) -> Permutations<T> {
        let size = items.len();
        Permutations {
            items,
            size,
            stack: vec![0; size],
            index: 0,
            started: false,
        }
    }
}

impl<T> FromIterator<T> for Permutations<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::new(Vec::from_iter(iter))
    }
}

impl<T: Clone> Iterator for Permutations<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>> {
        if self.started {
            while self.index < self.size {
                if self.stack[self.index] < self.index {
                    if self.index % 2 == 0 {
                        self.items.swap(0, self.index);
                    } else {
                        self.items.swap(self.stack[self.index], self.index);
                    }

                    self.stack[self.index] += 1;
                    self.index = 0;

                    return Some(self.items.clone());
                } else {
                    self.stack[self.index] = 0;
                    self.index += 1;
                }
            }

            None
        } else {
            self.started = true;
            Some(self.items.clone())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_permutations_of_list_with_size_3() {
        let mut permutations: HashSet<Vec<u8>> = HashSet::new();
        for permutation in Permutations::new(vec![1, 2, 3]) {
            permutations.insert(permutation.clone());
            assert_eq!(permutation.len(), 3);
            assert!(permutation.contains(&1));
            assert!(permutation.contains(&2));
            assert!(permutation.contains(&3));
        }
        assert_eq!(permutations.len(), 6);
    }

    #[test]
    fn test_permutations_of_list_with_size_5() {
        let mut permutations: HashSet<Vec<u8>> = HashSet::new();
        for permutation in (0..5).collect::<Permutations<u8>>() {
            permutations.insert(permutation.clone());
            assert_eq!(permutation.len(), 5);
            assert!(permutation.contains(&0));
            assert!(permutation.contains(&1));
            assert!(permutation.contains(&2));
            assert!(permutation.contains(&3));
            assert!(permutation.contains(&4));
        }
        assert_eq!(permutations.len(), 120);
    }
}
