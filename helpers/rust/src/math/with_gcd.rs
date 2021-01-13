use crate::math::gcd;

pub trait WithGcd<T> {
    fn gcd(&self) -> T;
}

impl WithGcd<u64> for Vec<u64> {
    fn gcd(&self) -> u64 {
        self.iter().fold(0, |acc, &c| gcd(acc, c))
    }
}

impl WithGcd<i64> for Vec<i64> {
    fn gcd(&self) -> i64 {
        self.iter().fold(0, |acc, &c| gcd(acc, c.abs()))
    }
}
