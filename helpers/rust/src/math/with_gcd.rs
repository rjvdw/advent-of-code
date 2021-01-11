use crate::math::gcd;

pub trait WithGcd {
    fn gcd(&self) -> u64;
}

impl WithGcd for Vec<u64> {
    fn gcd(&self) -> u64 {
        self.iter().fold(0, |acc, &c| gcd(acc, c))
    }
}

impl WithGcd for Vec<i64> {
    fn gcd(&self) -> u64 {
        self.iter().fold(0, |acc, &c| gcd(acc, c.abs() as u64))
    }
}
