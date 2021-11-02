use std::collections::HashSet;
use std::fs::File;
use std::ops::Add;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::input::WithReadLines;

fn main() {
    let args = get_args(&["<input file>"], 1);

    let directions = File::open(&args[1]).read_lines::<String>(1).next().unwrap();

    println!(
        "Santa ends up visiting only {} houses",
        follow_directions(&directions, 1)
    );
    println!(
        "Together with Robo-Santa, Santa ends up visiting {} houses",
        follow_directions(&directions, 2)
    );
}

fn follow_directions(directions: &str, nr_santas: usize) -> usize {
    let mut which_santa = 0;
    let mut positions: Vec<Position> = vec![Position::ORIGIN; nr_santas];
    let mut seen: HashSet<Position> = HashSet::new();
    seen.insert(Position::ORIGIN);

    for ch in directions.chars() {
        positions[which_santa] = match ch {
            '^' => positions[which_santa] + (1, 0),
            'v' => positions[which_santa] + (-1, 0),
            '>' => positions[which_santa] + (0, 1),
            '<' => positions[which_santa] + (0, -1),
            _ => positions[which_santa], // ignored
        };
        seen.insert(positions[which_santa]);
        which_santa = (which_santa + 1) % nr_santas;
    }

    seen.len()
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Position(i32, i32);

impl Position {
    const ORIGIN: Position = Position(0, 0);
}

impl Add<(i32, i32)> for Position {
    type Output = Position;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Position(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_follow_directions_1() {
        assert_eq!(follow_directions(">", 1), 2);
    }

    #[test]
    fn test_follow_directions_2() {
        assert_eq!(follow_directions("^>v<", 1), 4);
    }

    #[test]
    fn test_follow_directions_3() {
        assert_eq!(follow_directions("^v^v^v^v^v", 1), 2);
    }

    #[test]
    fn test_follow_directions_4() {
        assert_eq!(follow_directions("^v", 2), 3);
    }

    #[test]
    fn test_follow_directions_5() {
        assert_eq!(follow_directions("^>v<", 2), 3);
    }

    #[test]
    fn test_follow_directions_6() {
        assert_eq!(follow_directions("^v^v^v^v^v", 2), 11);
    }
}
