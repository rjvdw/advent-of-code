use crate::actor::Actor;
use crate::Cave;

pub(crate) mod drones;
pub(crate) mod you;

pub(crate) trait State {
    fn get_actors(&self) -> Vec<&Actor>;
    fn get_initial_state(cave: &Cave) -> Self;
    fn get_next_states(&self) -> Vec<(Self, Actor)>
    where
        Self: Sized;
    fn receive_key(&mut self, key: char);
    fn has_key(&self, key: char) -> bool;
    fn nr_keys(&self) -> usize;
}
