use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::direction::{CanTravel, Direction};
use rdcl_aoc_helpers::error::{ParseError, WithOrExit};
use rdcl_aoc_helpers::math::taxi_cab_2d;
use rdcl_aoc_helpers::parse_error;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let file = File::open(&args[1]).or_exit_with(1);
    let (wire1, wire2) = parse_input(file).or_exit_with(1);

    let (distance, steps) = find_closest_intersection(wire1, wire2);
    println!("The closest intersection is at distance {}.", distance);
    println!(
        "The fewest combined steps the wires must take is {}.",
        steps
    );
}

fn find_closest_intersection(wire1: Wire, wire2: Wire) -> (i64, usize) {
    let mut closest_to_origin = i64::MAX;
    let mut shortest_path = usize::MAX;
    let mut points: HashMap<(i64, i64), usize> = HashMap::new();

    for (idx, point) in wire1.enumerate() {
        points.insert(point, idx);
    }

    for (idx2, point) in wire2.enumerate() {
        if let Some(idx1) = points.get(&point) {
            let distance = taxi_cab_2d(point, (0, 0));
            if distance < closest_to_origin {
                closest_to_origin = distance;
            }

            if idx1 + idx2 + 2 < shortest_path {
                shortest_path = idx1 + idx2 + 2;
            }
        }
    }

    (closest_to_origin, shortest_path)
}

#[derive(Clone)]
struct Wire {
    sections: Vec<(Direction, usize)>,
    position: (i64, i64),
    section: usize,
    step: usize,
}

// impl Wire {
//     fn reset(&mut self) {
//         self.position = (0, 0);
//         self.section = 0;
//         self.step = 0;
//     }
// }

impl Iterator for Wire {
    type Item = (i64, i64);

    fn next(&mut self) -> Option<Self::Item> {
        while self.section < self.sections.len() && self.step >= self.sections[self.section].1 {
            self.section += 1;
            self.step = 0;
        }

        if self.section < self.sections.len() {
            let direction = self.sections[self.section].0;
            self.position = direction.travel(&self.position);
            self.step += 1;
            Some(self.position)
        } else {
            None
        }
    }
}

fn parse_input<R: Read>(r: R) -> Result<(Wire, Wire), ParseError> {
    let mut wires = vec![];
    for line in BufReader::new(r).lines() {
        let mut wire = Wire {
            sections: vec![],
            position: (0, 0),
            section: 0,
            step: 0,
        };

        let line = line?;
        for section in line.split(',') {
            let length = section[1..].parse()?;
            wire.sections.push(match &section[0..1] {
                "U" => (Direction::Up, length),
                "D" => (Direction::Down, length),
                "L" => (Direction::Left, length),
                "R" => (Direction::Right, length),
                _ => {
                    return Err(parse_error!("Invalid wire section: {}", section));
                }
            });
        }
        wires.push(wire);
    }
    Ok((wires[0].clone(), wires[1].clone()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_closest_intersection_1() {
        let test_input = vec!["R8,U5,L5,D3", "U7,R6,D4,L4"];
        let (wire1, wire2) = parse_input(test_input.join("\n").as_bytes()).unwrap();
        assert_eq!(find_closest_intersection(wire1, wire2), (6, 30));
    }

    #[test]
    fn test_find_closest_intersection_2() {
        let test_input = vec![
            "R75,D30,R83,U83,L12,D49,R71,U7,L72",
            "U62,R66,U55,R34,D71,R55,D58,R83",
        ];
        let (wire1, wire2) = parse_input(test_input.join("\n").as_bytes()).unwrap();
        assert_eq!(find_closest_intersection(wire1, wire2), (159, 610));
    }

    #[test]
    fn test_find_closest_intersection_3() {
        let test_input = vec![
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
            "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
        ];
        let (wire1, wire2) = parse_input(test_input.join("\n").as_bytes()).unwrap();
        assert_eq!(find_closest_intersection(wire1, wire2), (135, 410));
    }
}
