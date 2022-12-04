pub trait PolicyV2 {
    fn check(&self, pw: &str) -> bool;
}
