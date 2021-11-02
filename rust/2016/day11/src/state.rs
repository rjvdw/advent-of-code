use std::collections::HashMap;

use crate::item::Item;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Direction {
    Up,
    Down,
}

impl Direction {
    pub fn is_up(&self) -> bool {
        matches!(self, Direction::Up)
    }

    pub fn is_down(&self) -> bool {
        matches!(self, Direction::Down)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct State {
    pub floors: Vec<Vec<Item>>,
    pub floor: usize,
}

impl State {
    pub fn current_floor(&self) -> Vec<Item> {
        self.floors[self.floor].to_vec()
    }

    pub fn is_end_state(&self) -> bool {
        self.floors
            .iter()
            .take(self.floors.len() - 1)
            .all(|f| f.is_empty())
    }

    pub fn use_elevator(&self, direction: Direction, items: &[Item]) -> Option<State> {
        if (self.floor == 0 && direction.is_down())
            || (self.floor == self.floors.len() - 1 && direction.is_up())
            || !(items.len() == 1 || items.len() == 2)
            || !items.iter().all(|i| self.floors[self.floor].contains(i))
            || (direction.is_down() && self.floors.iter().take(self.floor).all(|f| f.is_empty()))
        {
            return None;
        }

        let next_current_floor = subtract(&self.floors[self.floor], items);

        if !Self::is_floor_valid(&next_current_floor) {
            return None;
        }

        let next_floor = match direction {
            Direction::Up => self.floor + 1,
            Direction::Down => self.floor - 1,
        };
        let mut next_next_floor = self.floors[next_floor].to_vec();
        for item in items {
            next_next_floor.push(item.clone());
        }

        if !Self::is_floor_valid(&next_next_floor) {
            return None;
        }

        let mut state = self.clone();
        state.floors[self.floor] = next_current_floor;
        state.floors[next_floor] = next_next_floor;
        state.floor = next_floor;

        state.floors = relabel_floors(&state.floors);

        Some(state)
    }

    pub fn is_floor_valid(floor: &[Item]) -> bool {
        let generators: Vec<String> = floor
            .iter()
            .filter(|i| i.is_generator())
            .map(|i| i.get_label())
            .collect();

        generators.is_empty()
            || floor
                .iter()
                .filter(|i| i.is_microchip())
                .map(|i| i.get_label())
                .all(|i| generators.contains(&i))
    }
}

fn subtract(p1: &[Item], p2: &[Item]) -> Vec<Item> {
    let mut p1 = p1.to_vec();
    let mut p2 = p2.to_vec();
    while let Some(v) = p2.pop() {
        p1.remove(p1.iter().position(|x| x.eq(&v)).unwrap());
    }
    p1
}

fn relabel_floors(floors: &[Vec<Item>]) -> Vec<Vec<Item>> {
    let mut floors = floors.to_vec();
    let mut label = 0;
    let mut mapping: HashMap<String, String> = HashMap::new();
    for floor in floors.iter_mut() {
        floor.sort_unstable();
        for item in floor.iter_mut() {
            let mapped = mapping.entry(item.get_label()).or_insert_with(|| {
                label += 1;
                label.to_string()
            });
            item.set_label(mapped.to_string());
        }
    }
    floors
}
