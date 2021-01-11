use std::cmp::Ordering;
use std::fmt;

use crate::math::with_gcd::WithGcd;

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
        } else if coefficients.len() == 1 {
            Polynomial {
                coefficients: coefficients.to_vec(),
            }
        } else {
            Polynomial {
                coefficients: coefficients
                    .iter()
                    .skip_while(|&&c| c == 0)
                    .copied()
                    .collect(),
            }
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
        let gcd = self.coefficients.gcd() as i64;
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

// TODO: Implement FnOnce, FnMut and Fn for Polynomial, so we can do `y(x)`, rather than `y.at(x)`.
// impl FnOnce(i64) for Polynomial {
//     type Output = i64;
//
//     fn call_once(self, x: i64) -> Self::Output {
//         self.at(x)
//     }
// }
//
// impl FnMut(i64) for Polynomial{
//     fn call_mut(&mut self, x: i64) -> Self::Output {
//         self.at(x)
//     }
// }
//
// impl Fn(i64) for Polynomial {
//     fn call(&self, x: i64) -> Self::Output {
//         self.at(x)
//     }
// }

impl fmt::Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let n = self.degree();
        if n == 0 {
            write!(f, "{}", self.coefficients[0])?;
        } else {
            write!(
                f,
                "{}{}",
                fmt_lead_coefficient(self.coefficients[0]),
                fmt_exponent(n)
            )?;
            for (i, &c) in self.coefficients.iter().enumerate().skip(1) {
                if c != 0 {
                    write!(f, "{}{}", fmt_coefficient(c), fmt_exponent(n - i))?;
                }
            }
        }
        Ok(())
    }
}

fn fmt_lead_coefficient(c: i64) -> String {
    match c {
        1 => String::new(),
        -1 => "-".to_string(),
        _ => c.to_string(),
    }
}

fn fmt_coefficient(c: i64) -> String {
    match c {
        1 => " + ".to_string(),
        -1 => " - ".to_string(),
        _ => match c.cmp(&0) {
            Ordering::Less => format!(" - {}", c.abs()),
            _ => format!(" + {}", c),
        },
    }
}

fn fmt_exponent(n: usize) -> String {
    match n {
        0 => String::new(),
        1 => "x".to_string(),
        _ => format!("x^{}", n),
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
