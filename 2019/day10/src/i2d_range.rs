//! The `I2dRange` ("Integer 2D Range") iterator takes two points (`from` and `to`), and returns
//! every (integer!) point that lies on a straight line between `from` and `to`.

use rdcl_aoc_helpers::math::{abs_diff, gcd};

/// The `I2dRange` ("Integer 2D Range") iterator takes two points (`from` and `to`), and returns
/// every (integer!) point that lies on a straight line between `from` and `to`.
#[derive(Debug, Copy, Clone)]
pub struct I2dRange {
    from: (i64, i64),
    to: (i64, i64),
    step_size: (i64, i64),
}

impl I2dRange {
    /// Creates a new `I2dRange` ("Integer 2D Range"). This is an iterator which takes two points,
    /// `from` and `to`, and returns every (integer!) point that lies on a straight line between
    /// `from` and `to`.
    pub fn new(from: (i64, i64), to: (i64, i64)) -> I2dRange {
        let dx = abs_diff(from.0, to.0);
        let dy = abs_diff(from.1, to.1);
        let g = gcd(dx, dy);
        let step_size = (
            if from.0 < to.0 { dx / g } else { -dx / g },
            if from.1 < to.1 { dy / g } else { -dy / g },
        );
        I2dRange {
            from,
            to: (to.0 - step_size.0, to.1 - step_size.1),
            step_size,
        }
    }
}

impl Iterator for I2dRange {
    type Item = (i64, i64);

    /// Return the next point that lies on a straight line between `self.from` and `self.to`.
    fn next(&mut self) -> Option<Self::Item> {
        if self.from != self.to {
            self.from.0 += self.step_size.0;
            self.from.1 += self.step_size.1;
            Some(self.from)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i2d_range_1() {
        let mut range = I2dRange::new((2, 3), (6, 5));
        assert_eq!(range.next(), Some((4, 4)));
        assert_eq!(range.next(), None);
    }

    #[test]
    fn test_i2d_range_2() {
        let mut range = I2dRange::new((9, 0), (0, 12));
        assert_eq!(range.next(), Some((6, 4)));
        assert_eq!(range.next(), Some((3, 8)));
        assert_eq!(range.next(), None);
    }
}
