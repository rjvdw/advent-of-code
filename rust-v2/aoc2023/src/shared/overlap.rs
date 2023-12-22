//! Logic pertaining to the overlap of objects.

/// When this trait is implemented, two objects can be checked for overlap.
pub trait Overlappable {
    /// Returns true if this object has overlap with some other object.
    fn overlaps_with(&self, other: Self) -> bool;
}

impl<T: Ord> Overlappable for (T, T) {
    fn overlaps_with(&self, other: Self) -> bool {
        self.1 >= other.0 && other.1 >= self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overlaps_with_for_tuples() {
        assert!((10, 15).overlaps_with((15, 20)));
        assert!((10, 15).overlaps_with((13, 20)));
        assert!((10, 15).overlaps_with((10, 20)));
        assert!((10, 15).overlaps_with((8, 20)));
        assert!((10, 15).overlaps_with((8, 15)));
        assert!((10, 15).overlaps_with((8, 13)));
        assert!((10, 15).overlaps_with((8, 10)));

        assert!(!(10, 15).overlaps_with((16, 20)));
        assert!(!(10, 15).overlaps_with((0, 5)));
    }
}
