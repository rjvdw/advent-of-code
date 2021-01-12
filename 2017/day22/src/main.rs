use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::direction::{CanTravel, Direction};
use rdcl_aoc_helpers::error::{ParseError, WithOrExit};

fn main() {
    let args = get_args(&["<input file>", "<v1 bursts>", "<v2 bursts>"], 1);
    let file = File::open(&args[1]).or_exit_with(1);
    let infected_nodes = parse_input(file).or_exit_with(1);
    let v1_bursts = args[2].replace('_', "").parse().or_exit_with(1);
    let v2_bursts = args[3].replace('_', "").parse().or_exit_with(1);

    println!(
        "After a total of {} bursts of activity, {} bursts will have caused an infection.",
        v1_bursts,
        carry_virus(&infected_nodes, v1_bursts, virus_v1)
    );

    println!(
        "After a total of {} bursts of activity, {} bursts will have caused an infection.",
        v2_bursts,
        carry_virus(&infected_nodes, v2_bursts, virus_v2)
    );
}

fn carry_virus<V>(infected_nodes: &HashMap<(i64, i64), Status>, bursts: usize, virus: V) -> usize
where
    V: Fn(Status, Direction) -> (Status, Direction),
{
    let mut infected_nodes = infected_nodes.clone();
    let mut infection_count = 0;
    let mut position = (0, 0);
    let mut direction = Direction::Up;

    for _ in 1..=bursts {
        let status = infected_nodes
            .get(&position)
            .copied()
            .unwrap_or(Status::Clean);

        let (new_status, new_direction) = virus(status, direction);

        if new_status.is_clean() {
            infected_nodes.remove(&position);
        } else {
            infected_nodes.insert(position, new_status);
        }

        if new_status.is_infected() {
            infection_count += 1;
        }

        direction = new_direction;
        position = direction.travel(&position);
    }

    infection_count
}

fn virus_v1(node: Status, direction: Direction) -> (Status, Direction) {
    match node {
        Status::Clean => (Status::Infected, direction.turn_left()),
        Status::Weakened => unimplemented!(),
        Status::Infected => (Status::Clean, direction.turn_right()),
        Status::Flagged => unimplemented!(),
    }
}

fn virus_v2(node: Status, direction: Direction) -> (Status, Direction) {
    match node {
        Status::Clean => (Status::Weakened, direction.turn_left()),
        Status::Weakened => (Status::Infected, direction),
        Status::Infected => (Status::Flagged, direction.turn_right()),
        Status::Flagged => (Status::Clean, direction.reverse()),
    }
}

fn parse_input<R: Read>(readable: R) -> Result<HashMap<(i64, i64), Status>, ParseError> {
    let mut parsed = vec![];
    let mut height = 0;
    let mut width = 0;
    for (y, line) in BufReader::new(readable).lines().enumerate() {
        height += 1;
        let line = line?;
        for (x, ch) in line.chars().enumerate() {
            if y == 0 {
                width += 1;
            }
            if ch == '#' {
                parsed.push((x as i64, y as i64));
            }
        }
    }

    height /= 2;
    width /= 2;

    let mut infected_nodes = HashMap::new();
    for (x, y) in parsed {
        infected_nodes.insert((x - width, y - height), Status::Infected);
    }
    Ok(infected_nodes)
}

#[derive(Debug, Copy, Clone)]
enum Status {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

impl Status {
    fn is_clean(&self) -> bool {
        matches!(self, Status::Clean)
    }

    fn is_infected(&self) -> bool {
        matches!(self, Status::Infected)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_virus_v1() {
        let test_input = vec!["..#", "#..", "..."];
        let infected_nodes = parse_input(test_input.join("\n").as_bytes()).unwrap();

        assert_eq!(carry_virus(&infected_nodes, 70, virus_v1), 41);
    }

    #[test]
    fn test_virus_v2() {
        let test_input = vec!["..#", "#..", "..."];
        let infected_nodes = parse_input(test_input.join("\n").as_bytes()).unwrap();

        assert_eq!(carry_virus(&infected_nodes, 100, virus_v2), 26);
    }
}
