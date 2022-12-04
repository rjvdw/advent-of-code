pub trait PolicyV1 {
    fn check(&self, pw: &str) -> bool;
}
