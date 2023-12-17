use std::collections::{HashMap, HashSet};

use rdcl_aoc_pathfinding::AStar;

use crate::label::Label;

#[derive(Debug, Default)]
pub struct Tunnels {
    t: HashMap<Label, HashSet<Label>>,
}

impl Tunnels {
    pub fn insert(&mut self, label: Label, targets: HashSet<Label>) {
        self.t.insert(label, targets);
    }
}

impl AStar for Tunnels {
    type Point = Label;
    type EndPoint = Label;

    fn distance_score(&self, _: &Self::Point, _: &Self::Point) -> u64 {
        1 // TODO: Just use Dijkstra...
    }

    fn get_neighbours(&self, point: &Self::Point) -> Vec<(u64, Self::Point)> {
        match self.t.get(point) {
            Some(t) => t.iter().copied().map(|v| (1, v)).collect(),
            None => vec![],
        }
    }
}
