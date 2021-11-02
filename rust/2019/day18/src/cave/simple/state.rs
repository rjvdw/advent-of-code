use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub(in crate::cave::simple) struct State {
    pub(in crate::cave::simple) position: (usize, usize),
    pub(in crate::cave::simple) keys: Vec<char>,
    pub(in crate::cave::simple) distance: usize,
}

pub(in crate::cave::simple) struct HashedState(pub(in crate::cave::simple) State);

impl PartialEq for HashedState {
    fn eq(&self, other: &Self) -> bool {
        self.0.position.eq(&other.0.position) && self.0.keys.eq(&other.0.keys)
    }
}

impl Eq for HashedState {}

impl Hash for HashedState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.position.hash(state);
        self.0.keys.hash(state);
    }
}

pub(in crate::cave::simple) struct ScoredState(pub(in crate::cave::simple) State);

impl PartialEq for ScoredState {
    fn eq(&self, other: &Self) -> bool {
        self.0.distance.eq(&other.0.distance)
    }
}

impl Eq for ScoredState {}

impl PartialOrd for ScoredState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ScoredState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.distance.cmp(&other.0.distance).reverse()
    }
}
