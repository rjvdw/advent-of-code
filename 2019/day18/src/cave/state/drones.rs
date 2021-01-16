use std::fmt;
use std::fmt::{Display, Formatter};

use crate::actor::Actor;
use crate::state::State;
use crate::Cave;

macro_rules! next_state {
    ($self:expr, $states:expr, $x:tt) => {
        for neighbour in $self.drones.$x.get_neighbours() {
            let mut state = $self.clone();
            state.drones.$x = neighbour.clone();
            $states.push((state, neighbour));
        }
    };
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct WithFourDrones {
    drones: (Actor, Actor, Actor, Actor),
    recovered_keys: Vec<char>,
}

impl State for WithFourDrones {
    fn get_actors(&self) -> Vec<&Actor> {
        vec![
            &self.drones.0,
            &self.drones.1,
            &self.drones.2,
            &self.drones.3,
        ]
    }

    fn get_initial_state(cave: &Cave) -> Self {
        let mut drones = (
            Actor::default(),
            Actor::default(),
            Actor::default(),
            Actor::default(),
        );

        for y in 0..cave.layout.rows() {
            for x in 0..cave.layout.cols() {
                let drone = if y < cave.layout.rows() / 2 {
                    if x < cave.layout.cols() / 2 {
                        &mut drones.0
                    } else {
                        &mut drones.1
                    }
                } else if x < cave.layout.cols() / 2 {
                    &mut drones.2
                } else {
                    &mut drones.3
                };
                drone.update_from_tile(cave.layout[y][x], (x, y));
            }
        }

        WithFourDrones {
            drones,
            recovered_keys: vec![],
        }
    }

    fn get_next_states(&self) -> Vec<(Self, Actor)>
    where
        Self: Sized,
    {
        let mut states = vec![];
        next_state!(self, states, 0);
        next_state!(self, states, 1);
        next_state!(self, states, 2);
        next_state!(self, states, 3);
        states
    }

    fn receive_key(&mut self, key: char) {
        if !self.has_key(key) {
            self.recovered_keys.push(key);
            self.recovered_keys.sort_unstable();
        }
        self.drones.0.receive_key(key);
        self.drones.1.receive_key(key);
        self.drones.2.receive_key(key);
        self.drones.3.receive_key(key);
    }

    fn has_key(&self, key: char) -> bool {
        self.recovered_keys.contains(&key)
    }

    fn nr_keys(&self) -> usize {
        self.recovered_keys.len()
    }
}

impl Display for WithFourDrones {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "The drones are at: {:?}, {:?}, {:?}, {:?}. The following keys have been recovered: {:?}.",
            self.drones.0.position,
            self.drones.1.position,
            self.drones.2.position,
            self.drones.3.position,
            self.recovered_keys,
        )
    }
}
