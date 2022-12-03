//! Represents something which can be scored.

pub trait WithScore {
    fn get_score(&self) -> u32;
}
