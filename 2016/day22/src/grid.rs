use std::collections::HashMap;

use rdcl_aoc_helpers::search::Navigable;

use crate::node::Node;

pub struct Grid {
    pub start: Node,
    pub target: Node,
    pub width: usize,
    map: HashMap<(usize, usize), Node>,
}

impl Grid {
    pub fn new(nodes: &[Node]) -> Grid {
        let mut start = nodes[0]; // placeholder
        let mut target = nodes[0]; // placeholder
        let mut width = 0; // placeholder

        let mut map: HashMap<(usize, usize), Node> = HashMap::new();
        for &node in nodes {
            map.insert(node.get_xy(), node);

            let (x, y) = node.get_xy();
            if node.is_empty() {
                start = node;
            }

            if y == 0 && x + 1 > width {
                width = x + 1;
                target = node;
            }
        }

        Grid {
            start,
            target,
            width,
            map,
        }
    }
}

impl Navigable for Grid {
    type Point = Node;

    fn distance_score(&self, a: &Self::Point, b: &Self::Point) -> u64 {
        a.distance(b) as u64
    }

    fn get_neighbours(&self, node: &Self::Point) -> Vec<(u64, Self::Point)> {
        let (x, y) = node.get_xy();
        let mut neighbours: Vec<(usize, usize)> = vec![(x + 1, y), (x, y + 1)];
        if x > 0 {
            neighbours.push((x - 1, y))
        }
        if y > 0 {
            neighbours.push((x, y - 1))
        }

        neighbours
            .iter()
            .map(|xy| self.map.get(xy))
            .filter(|node_opt| node_opt.is_some())
            .map(|node_opt| node_opt.unwrap())
            .filter(|node1| node1.fits_on(&self.start))
            .map(|&node| (1, node))
            .collect()
    }
}
