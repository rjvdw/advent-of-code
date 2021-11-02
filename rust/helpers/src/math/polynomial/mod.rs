use crate::math::with_gcd::WithGcd;

mod impl_arithmetics;
mod impl_fmt;
mod impl_fn;

/// A polynomial.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Polynomial {
    coefficients: Vec<i64>,
}

impl Polynomial {
    /// The coefficients of the polynomial. The left-most coefficient corresponds with x^n. The
    /// right-most with x^0.
    pub fn new(coefficients: &[i64]) -> Polynomial {
        if coefficients.is_empty() {
            Polynomial {
                coefficients: vec![0],
            }
        } else {
            let mut coefficients: Vec<i64> = coefficients
                .iter()
                .skip_while(|&&c| c == 0)
                .copied()
                .collect();

            if coefficients.is_empty() {
                coefficients.push(0);
            }

            Polynomial { coefficients }
        }
    }

    /// The degree of a polynomial. I.e. the highest n such that the coefficient for x^n is not
    /// zero.
    pub fn degree(&self) -> usize {
        self.coefficients.len() - 1
    }

    /// Find integer values x, such that the polynomial evaluates to 0 for x. These values are
    /// sorted from smallest to largest.
    pub fn find_roots(&self) -> Vec<i64> {
        let gcd = self.coefficients.gcd();
        let last_coefficient = self
            .coefficients
            .iter()
            .rev()
            .find(|&&c| c != 0)
            .map(|&c| c / gcd);

        let mut roots = if let Some(p) = last_coefficient {
            (1..=p.abs())
                .filter(|&x| p % x == 0)
                .flat_map(|x| vec![x, -x])
                .filter(|&x| self.at(x) == 0)
                .collect()
        } else {
            vec![]
        };

        if self.coefficients.last().copied().unwrap() == 0 {
            roots.push(0);
        }

        roots.sort_unstable();
        roots
    }

    /// Evaluate the polynomial for x.
    pub fn at(&self, x: i64) -> i64 {
        let mut i = 1;
        let mut y = 0;
        for a in self.coefficients.iter().rev() {
            y += a * i;
            i *= x;
        }
        y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_roots_1() {
        // y = 7 (x - 10) (x - 3)
        let polynomial = Polynomial::new(&[7, -91, 210]);
        assert_eq!(polynomial.find_roots(), vec![3, 10]);
    }

    #[test]
    fn test_find_roots_2() {
        // y = 7 (x - 10) x
        let polynomial = Polynomial::new(&[7, -70, 0]);
        assert_eq!(polynomial.find_roots(), vec![0, 10]);
    }

    #[test]
    fn test_find_roots_3() {
        // y = x^2
        let polynomial = Polynomial::new(&[1, 0, 0]);
        assert_eq!(polynomial.find_roots(), vec![0]);
    }

    #[test]
    fn test_find_roots_4() {
        // y = 10 (x - 5) (x - 1/2)
        let polynomial = Polynomial::new(&[10, -55, 25]);
        assert_eq!(polynomial.find_roots(), vec![5]);
    }

    #[test]
    fn test_find_roots_5() {
        // y = 10 (x - 1/5) (x - 1/2)
        let polynomial = Polynomial::new(&[10, -7, 1]);
        assert_eq!(polynomial.find_roots(), vec![]);
    }

    #[test]
    fn test_add_two_polynomials_of_equal_degree() {
        let p1 = Polynomial::new(&[1, 2, 3]);
        let p2 = Polynomial::new(&[10, 20, 30]);
        let p3 = Polynomial::new(&[11, 22, 33]);

        assert_eq!(p1 + p2, p3);
    }

    #[test]
    fn test_add_two_polynomials_with_different_degrees() {
        let p1 = Polynomial::new(&[1, 2, 3, 4]);
        let p2 = Polynomial::new(&[10, 20, 30]);
        let p3 = Polynomial::new(&[1, 12, 23, 34]);

        assert_eq!(p1 + p2, p3);
    }

    #[test]
    fn test_add_opposite_polynomials() {
        let p1 = Polynomial::new(&[1, 2, 3]);
        let p2 = Polynomial::new(&[-1, -2, -3]);
        let p3 = Polynomial::new(&[0]);

        assert_eq!(p1 + p2, p3);
    }

    #[test]
    fn test_neg_polynomials() {
        let p1 = Polynomial::new(&[1, 2, 3]);
        let p2 = Polynomial::new(&[-1, -2, -3]);

        assert_eq!(-p1, p2);
    }

    #[test]
    fn test_subtract_polynomials() {
        let p1 = Polynomial::new(&[10, 20, 30]);
        let p2 = Polynomial::new(&[1, 2, 3, 4]);
        let p3 = Polynomial::new(&[-1, 8, 17, 26]);

        assert_eq!(p1 - p2, p3);
    }

    #[test]
    fn test_formatting() {
        assert_fmt(&[], "0");
        assert_fmt(&[0], "0");
        assert_fmt(&[1], "1");
        assert_fmt(&[-1], "-1");
        assert_fmt(&[1, 2], "x + 2");
        assert_fmt(&[-1, 2], "-x + 2");
        assert_fmt(&[2, 2], "2x + 2");
        assert_fmt(&[2, -2], "2x - 2");
        assert_fmt(&[-2, -2], "-2x - 2");
        assert_fmt(&[-2, 0], "-2x");
        assert_fmt(&[1, 2, 3], "x^2 + 2x + 3");
        assert_fmt(&[1, 0, 3], "x^2 + 3");
    }

    fn assert_fmt(coefficients: &[i64], expected: &str) {
        let polynomial = Polynomial::new(coefficients);
        println!("Testing '{}' == '{}'", polynomial, expected);
        assert_eq!(format!("{}", polynomial), expected);
    }
}
