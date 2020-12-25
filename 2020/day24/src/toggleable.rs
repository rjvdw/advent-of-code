use std::collections::HashSet;
use std::hash::Hash;

pub trait Toggleable<T> {
    fn toggle(&mut self, key: &T);
}

impl<T: Eq + Hash + Copy> Toggleable<T> for HashSet<T> {
    fn toggle(&mut self, key: &T) {
        if self.contains(key) {
            self.remove(key);
        } else {
            self.insert(*key);
        }
    }
}
