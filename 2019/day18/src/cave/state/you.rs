use std::fmt;
use std::fmt::{Display, Formatter};

use crate::actor::Actor;
use crate::state::State;
use crate::Cave;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct WithJustYou {
    you: Actor,
    recovered_keys: usize,
}

impl State for WithJustYou {
    fn get_actors(&self) -> Vec<&Actor> {
        vec![&self.you]
    }

    fn get_initial_state(cave: &Cave) -> Self {
        let mut you = Actor::default();

        // find the entrance and all keys
        for y in 0..cave.layout.rows() {
            for x in 0..cave.layout.cols() {
                you.update_from_tile(cave.layout[y][x], (x, y));
            }
        }

        WithJustYou {
            you,
            recovered_keys: 0,
        }
    }

    fn get_next_states(&self) -> Vec<(Self, Actor)>
    where
        Self: Sized,
    {
        let mut states = vec![];
        for neighbour in self.you.get_neighbours() {
            states.push((
                WithJustYou {
                    you: neighbour.clone(),
                    ..self.clone()
                },
                neighbour,
            ))
        }
        states
    }

    fn receive_key(&mut self, key: char) {
        if !self.has_key(key) {
            self.recovered_keys += 1;
        }
        self.you.receive_key(key);
    }

    fn has_key(&self, key: char) -> bool {
        self.you.has_key(key)
    }

    fn nr_keys(&self) -> usize {
        self.recovered_keys
    }
}

impl Display for WithJustYou {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "You are at {:?}, and have the following keys: {:?}",
            self.you.position, self.you.recovered_keys
        )
    }
}
