use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;
use std::io::Read;
use std::ops::Add;
use std::str::FromStr;

use rdcl_aoc_helpers::input::MappedLines;

pub type Graph<K, V> = HashMap<K, HashMap<K, V>>;

pub trait EdgeLike<K, V> {
    fn from(&self) -> K;
    fn to(&self) -> K;
    fn value(&self) -> V;
}

pub fn create_graph<K, V, E, R>(edges: MappedLines<E, R>, bidirectional: bool) -> Graph<K, V>
where
    K: Eq + Hash,
    E: EdgeLike<K, V> + FromStr,
    <E as FromStr>::Err: fmt::Debug,
    R: Read,
{
    let mut graph = HashMap::new();
    for edge in edges {
        graph
            .entry(edge.from())
            .or_insert_with(HashMap::new)
            .insert(edge.to(), edge.value());

        if bidirectional {
            graph
                .entry(edge.to())
                .or_insert_with(HashMap::new)
                .insert(edge.from(), edge.value());
        }
    }
    graph
}

pub fn find_shortest_path<K, V>(graph: &Graph<K, V>, circular: bool) -> Option<(Vec<K>, V)>
where
    K: Eq + Hash + Clone,
    V: Default + Ord + Add<Output = V> + Copy,
    <V as Add>::Output: Ord,
{
    find_path(graph, circular, &MinOrMax::Min)
}

pub fn find_longest_path<K, V>(graph: &Graph<K, V>, circular: bool) -> Option<(Vec<K>, V)>
where
    K: Eq + Hash + Clone,
    V: Default + Ord + Add<Output = V> + Copy,
    <V as Add>::Output: Ord,
{
    find_path(graph, circular, &MinOrMax::Max)
}

fn find_path<K, V>(
    graph: &Graph<K, V>,
    circular: bool,
    distance_req: &MinOrMax,
) -> Option<(Vec<K>, V)>
where
    K: Eq + Hash + Clone,
    V: Default + Ord + Add<Output = V> + Copy,
    <V as Add>::Output: Ord,
{
    let iter = graph
        .keys()
        .map(|starting_node| {
            traverse_graph(
                graph,
                &[starting_node.clone()],
                Default::default(),
                circular,
                distance_req,
            )
        })
        .flatten();

    match distance_req {
        MinOrMax::Min => iter.min_by_key(|(_, distance)| *distance),
        MinOrMax::Max => iter.max_by_key(|(_, distance)| *distance),
    }
}

fn traverse_graph<K, V>(
    graph: &Graph<K, V>,
    traveled: &[K],
    distance_so_far: V,
    circular: bool,
    distance_req: &MinOrMax,
) -> Option<(Vec<K>, V)>
where
    K: Eq + Hash + Clone,
    V: Default + Ord + Add<Output = V> + Copy,
    <V as Add>::Output: Ord,
{
    let iter = graph
        .get(traveled.iter().last()?)?
        .iter()
        .filter(|(node, _)| !traveled.contains(node))
        .map(|(node, distance)| {
            let mut new_traveled = traveled.to_vec();
            new_traveled.push(node.clone());
            (new_traveled, distance_so_far + *distance)
        });

    if traveled.len() + 1 == graph.len() {
        if circular {
            let iter = iter
                .map(|(path, distance)| {
                    if path.is_empty() {
                        Some((path, distance))
                    } else {
                        let first = path.first().unwrap();
                        let last = path.last().unwrap();

                        graph
                            .get(last)
                            .map(|sg| sg.get(first))
                            .flatten()
                            .map(|d| (path, distance + *d))
                    }
                })
                .flatten();

            match distance_req {
                MinOrMax::Min => iter.min_by_key(|(_, distance)| *distance),
                MinOrMax::Max => iter.max_by_key(|(_, distance)| *distance),
            }
        } else {
            match distance_req {
                MinOrMax::Min => iter.min_by_key(|(_, distance)| *distance),
                MinOrMax::Max => iter.max_by_key(|(_, distance)| *distance),
            }
        }
    } else {
        let iter = iter
            .map(|(traveled, distance)| {
                traverse_graph(graph, &traveled, distance, circular, distance_req)
            })
            .flatten();

        match distance_req {
            MinOrMax::Min => iter.min_by_key(|(_, distance)| *distance),
            MinOrMax::Max => iter.max_by_key(|(_, distance)| *distance),
        }
    }
}

enum MinOrMax {
    Min,
    Max,
}
