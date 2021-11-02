use std::ops::{Add, Index, Mul, Neg, Sub};

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

impl Add<i64> for Polynomial {
    type Output = Polynomial;

    fn add(self, rhs: i64) -> Self::Output {
        let mut p = self;
        *p.coefficients.last_mut().unwrap() += rhs;
        p
    }
}

impl Sub<i64> for Polynomial {
    type Output = Polynomial;

    fn sub(self, rhs: i64) -> Self::Output {
        self + (-rhs)
    }
}

impl Mul<i64> for Polynomial {
    type Output = Polynomial;

    fn mul(self, rhs: i64) -> Self::Output {
        let mut p = self;
        for c in &mut p.coefficients {
            *c *= rhs;
        }
        p
    }
}

impl Index<usize> for Polynomial {
    type Output = i64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.coefficients[index]
    }
}
