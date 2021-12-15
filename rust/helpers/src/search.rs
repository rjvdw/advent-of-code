//! Searching algorithms.
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;
use std::ops::Add;

/// This trait allows searching algorithms to navigate a space.
pub trait Navigable {
    /// Describes a single point in the searchable space.
    type Point;

    /// The distance score between points a and b.
    fn distance_score(&self, a: &Self::Point, b: &Self::Point) -> u64;

    /// Returns the points that can be reached directly from `point`, together with the distance.
    fn get_neighbours(&self, point: &Self::Point) -> Vec<(u64, Self::Point)>;

    /// Use [A*](https://en.wikipedia.org/wiki/A*_search_algorithm) to find the shortest path
    /// between two points within a `Navigable` space.
    fn find_shortest_path(&self, start: &Self::Point, end: &Self::Point) -> Option<Vec<Self::Point>>
    where
        Self::Point: Hash + PartialEq + Eq + Clone,
    {
        let mut open_set: BinaryHeap<SortablePoint<Self::Point>> = BinaryHeap::new();
        open_set.push(SortablePoint {
            point: start.clone(),
            f_score: self.distance_score(start, end),
        });

        let mut came_from: HashMap<Self::Point, Self::Point> = HashMap::new();

        let mut g_score: HashMap<Self::Point, u64> = HashMap::new();
        g_score.insert(start.clone(), 0);

        while let Some(current) = open_set.pop() {
            let current = &current.point;
            if current == end {
                let mut point = current;
                let mut path = vec![point.clone()];
                while let Some(p) = came_from.get(point) {
                    point = p;
                    path.push(p.clone());
                }
                path.reverse();
                return Some(path);
            }

            let current_distance = *g_score.get(current).unwrap_or(&u64::MAX);

            for (d, neighbour) in &self.get_neighbours(current) {
                let distance = current_distance + d;
                let neighbour_distance = *g_score.get(neighbour).unwrap_or(&u64::MAX);

                if distance < neighbour_distance {
                    came_from.insert(neighbour.clone(), current.clone());
                    g_score.insert(neighbour.clone(), distance);
                    open_set.push(SortablePoint {
                        point: neighbour.clone(),
                        f_score: distance.add(self.distance_score(neighbour, end)),
                    });
                }
            }
        }

        None
    }
}

/// In the A* implementation, a priority queue (i.e. BinaryHeap) is used. This wrapper around the
/// points allows sorting on the f_score. Note that a descending sort is used.
struct SortablePoint<P> {
    point: P,
    f_score: u64,
}

impl<P> Eq for SortablePoint<P> {}

impl<P> PartialEq<Self> for SortablePoint<P> {
    fn eq(&self, other: &Self) -> bool {
        other.f_score.eq(&self.f_score)
    }
}

impl<P> PartialOrd<Self> for SortablePoint<P> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.f_score.partial_cmp(&self.f_score)
    }
}

impl<P> Ord for SortablePoint<P> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_score.cmp(&self.f_score)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::math::taxi_cab_2d;

    use super::*;

    #[test]
    fn test_find_shortest_path_without_obstacles() {
        // ############
        // #S.........#
        // #..........#
        // #..........#
        // #..........#
        // #.........E#
        // ############

        let nav = TestNav::new(10, 5, &[]);
        let path = nav.find_shortest_path(&(1, 1), &(10, 5));
        assert_eq!(path.map(|p| p.len()), Some(14));
    }

    #[test]
    fn test_find_shortest_with_obstacles() {
        // ############
        // #S#.....#..#
        // #.###.#...##
        // #.....#.#..#
        // #..#######.#
        // #.....#...E#
        // ############

        let obstacles = vec![
            (2, 1),
            (2, 2),
            (3, 2),
            (3, 4),
            (4, 2),
            (4, 4),
            (5, 4),
            (6, 2),
            (6, 3),
            (6, 4),
            (6, 5),
            (7, 4),
            (8, 1),
            (8, 3),
            (8, 4),
            (9, 4),
            (10, 2),
        ];
        let nav = TestNav::new(10, 5, &obstacles);
        let path = nav.find_shortest_path(&(1, 1), &(10, 5));
        assert_eq!(
            path,
            Some(vec![
                (1, 1),
                (1, 2),
                (1, 3),
                (2, 3),
                (3, 3),
                (4, 3),
                (5, 3),
                (5, 2),
                (5, 1),
                (6, 1),
                (7, 1),
                (7, 2),
                (8, 2),
                (9, 2),
                (9, 3),
                (10, 3),
                (10, 4),
                (10, 5),
            ])
        );
    }

    #[test]
    fn test_find_shortest_path_impossible() {
        // ############
        // #S.........#
        // #..........#
        // ############
        // #..........#
        // #.........E#
        // ############

        let obstacles = vec![
            (1, 3),
            (2, 3),
            (3, 3),
            (4, 3),
            (5, 3),
            (6, 3),
            (7, 3),
            (8, 3),
            (9, 3),
            (10, 3),
        ];
        let nav = TestNav::new(10, 10, &obstacles);
        let path = nav.find_shortest_path(&(1, 1), &(10, 5));
        assert_eq!(path, None);
    }

    struct TestNav {
        width: u64,
        height: u64,
        obstacles: HashSet<(u64, u64)>,
    }

    impl TestNav {
        fn new(width: u64, height: u64, obstacles: &[(u64, u64)]) -> TestNav {
            let mut obstacles_as_set = HashSet::new();
            for obstacle in obstacles {
                obstacles_as_set.insert(*obstacle);
            }
            TestNav {
                width,
                height,
                obstacles: obstacles_as_set,
            }
        }
    }

    impl Navigable for TestNav {
        type Point = (u64, u64);

        fn distance_score(&self, a: &Self::Point, b: &Self::Point) -> u64 {
            taxi_cab_2d(*a, *b)
        }

        fn get_neighbours(&self, (x, y): &Self::Point) -> Vec<(u64, Self::Point)> {
            vec![(*x - 1, *y), (*x + 1, *y), (*x, *y - 1), (*x, *y + 1)]
                .iter()
                .filter(|(x, y)| *x > 0 && *x <= self.width && *y > 0 && *y <= self.height)
                .filter(|p| !self.obstacles.contains(p))
                .copied()
                .map(|p| (1, p))
                .collect()
        }
    }
}
