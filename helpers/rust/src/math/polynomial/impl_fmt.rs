use std::cmp::Ordering;
use std::fmt;

use crate::math::polynomial::Polynomial;

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
