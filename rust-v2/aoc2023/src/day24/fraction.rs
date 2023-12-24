use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Sub};

use rdcl_aoc_math::gcd;

/// A normalized fractional value.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Fraction(i64, i64);

impl Fraction {
    /// Construct a new fraction.
    pub fn new(mut numerator: i64, mut denominator: i64) -> Fraction {
        if denominator == 0 {
            panic!("Division by zero!");
        }

        let g = gcd(numerator, denominator);
        numerator /= g;
        denominator /= g;

        if denominator < 0 {
            numerator *= -1;
            denominator *= -1;
        }

        Fraction(numerator, denominator)
    }

    /// Check if a fraction is between to integer values.
    pub fn is_between(&self, lower: i64, upper: i64) -> bool {
        self >= &lower && self <= &upper
    }
}

impl Add<i64> for Fraction {
    type Output = Fraction;

    fn add(self, rhs: i64) -> Self::Output {
        Fraction::new(self.0 + (rhs * self.1), self.1)
    }
}

impl Sub for Fraction {
    type Output = Fraction;

    fn sub(self, rhs: Self) -> Self::Output {
        Fraction::new((self.0 * rhs.1) - (rhs.0 * self.1), self.1 * rhs.1)
    }
}

impl Mul for Fraction {
    type Output = Fraction;

    fn mul(self, rhs: Self) -> Self::Output {
        Fraction::new(self.0 * rhs.0, self.1 * rhs.1)
    }
}

impl Mul<i64> for Fraction {
    type Output = Fraction;

    fn mul(self, rhs: i64) -> Self::Output {
        Fraction::new(self.0 * rhs, self.1)
    }
}

impl Div for Fraction {
    type Output = Fraction;

    fn div(self, rhs: Self) -> Self::Output {
        Fraction::new(self.0 * rhs.1, self.1 * rhs.0)
    }
}

impl PartialEq<i64> for Fraction {
    fn eq(&self, other: &i64) -> bool {
        self.1 == 1 && self.0 == *other
    }
}

impl PartialOrd<i64> for Fraction {
    fn partial_cmp(&self, other: &i64) -> Option<Ordering> {
        Some(self.0.cmp(&(*other * self.1)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(Fraction::new(1, 2), Fraction::new(2, 4));
        assert_eq!(Fraction::new(2, 4), Fraction::new(4, 8));
        assert_eq!(Fraction::new(2, 2), Fraction::new(1, 1));
        assert_eq!(Fraction::new(0, 2), Fraction::new(0, 1));
        assert_eq!(Fraction::new(-1, 2), Fraction::new(1, -2));
    }

    #[test]
    fn test_addition() {
        assert_eq!(Fraction::new(1, 2) + 1, Fraction(3, 2));
    }

    #[test]
    fn test_subtraction() {
        assert_eq!(
            Fraction::new(3, 2) - Fraction::new(1, 2),
            Fraction::new(1, 1)
        );
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(Fraction::new(1, 3) * 2, Fraction::new(2, 3));
        assert_eq!(
            Fraction::new(1, 3) * Fraction::new(6, 5),
            Fraction::new(2, 5)
        );
    }

    #[test]
    fn test_division() {
        assert_eq!(
            Fraction::new(2, 5) / Fraction::new(6, 5),
            Fraction::new(1, 3)
        );
    }

    #[test]
    fn test_equality() {
        assert_eq!(Fraction::new(5, 1), 5);
    }

    #[test]
    fn test_ordering() {
        assert!(Fraction::new(1, 2) < 1);
        assert!(Fraction::new(1, 2) > 0);
        assert!(Fraction::new(-1, 2) < 0);
        assert!(Fraction::new(-1, 2) > -1);
        assert!(Fraction::new(5, 1) <= 5);
        assert!(Fraction::new(5, 1) >= 5);
        assert!(Fraction::new(-5, 1) <= -5);
        assert!(Fraction::new(-5, 1) >= -5);
    }
}
