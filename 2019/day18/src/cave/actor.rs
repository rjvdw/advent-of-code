use std::ops::Index;

use grid::Grid;

use crate::tile::Tile;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub(crate) struct Actor {
    pub(crate) position: (usize, usize),
    pub(crate) recovered_keys: Vec<char>,
    pub(crate) keys_to_recover: Vec<char>,
    pub(crate) doors_to_unlock: Vec<char>,
}

impl Actor {
    pub(crate) fn update_from_tile(&mut self, tile: Tile, position: (usize, usize)) {
        match tile {
            Tile::Entrance => {
                self.position = position;
            }
            Tile::Door(key) => {
                self.doors_to_unlock.push(key);
                self.doors_to_unlock.sort_unstable();
            }
            Tile::Key(key) => {
                self.keys_to_recover.push(key);
                self.keys_to_recover.sort_unstable();
            }
            _ => {}
        }
    }

    pub(crate) fn is_done(&self) -> bool {
        self.keys_to_recover
            .iter()
            .all(|key| self.recovered_keys.contains(key))
    }

    pub(crate) fn receive_key(&mut self, key: char) {
        if self.keys_to_recover.contains(&key) || self.doors_to_unlock.contains(&key) {
            self.recovered_keys.push(key);
            self.recovered_keys.sort_unstable();
        }
    }

    pub(crate) fn has_key(&self, key: char) -> bool {
        self.recovered_keys.contains(&key)
    }

    pub(crate) fn get_neighbours(&self) -> Vec<Actor> {
        // assumption: We are surrounded by walls so will never reach (0, _) or (_, 0)
        // therefore, we do not need bound checks

        let (x, y) = self.position;
        vec![
            Actor {
                position: (x, y - 1),
                ..self.clone()
            },
            Actor {
                position: (x, y + 1),
                ..self.clone()
            },
            Actor {
                position: (x - 1, y),
                ..self.clone()
            },
            Actor {
                position: (x + 1, y),
                ..self.clone()
            },
        ]
    }
}

impl Default for Actor {
    fn default() -> Self {
        Actor {
            position: (0, 0),
            recovered_keys: vec![],
            keys_to_recover: vec![],
            doors_to_unlock: vec![],
        }
    }
}

impl Index<&Actor> for Grid<Tile> {
    type Output = Tile;

    fn index(&self, actor: &Actor) -> &Self::Output {
        &self[actor.position.1][actor.position.0]
    }
}
