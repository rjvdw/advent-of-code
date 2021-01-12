use std::ops::{Add, Neg, Sub};

use crate::math::polynomial::Polynomial;

impl Add<Polynomial> for Polynomial {
    type Output = Polynomial;

    fn add(self, rhs: Polynomial) -> Self::Output {
        let new_len = self.coefficients.len().max(rhs.coefficients.len());
        let mut coefficients = vec![0; new_len];
        let mut c1 = self.coefficients.iter().rev();
        let mut c2 = rhs.coefficients.iter().rev();
        for c in coefficients.iter_mut().rev() {
            *c = *c1.next().unwrap_or(&0) + *c2.next().unwrap_or(&0);
        }
        Polynomial::new(&coefficients)
    }
}

impl Sub<Polynomial> for Polynomial {
    type Output = Polynomial;

    fn sub(self, rhs: Polynomial) -> Self::Output {
        self.add(rhs.neg())
    }
}

impl Neg for Polynomial {
    type Output = Polynomial;

    fn neg(self) -> Self::Output {
        let coefficients: Vec<i64> = self.coefficients.iter().map(|&c| -c).collect();
        Polynomial::new(&coefficients)
    }
}
