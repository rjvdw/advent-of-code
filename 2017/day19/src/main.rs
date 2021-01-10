use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;
use rdcl_aoc_helpers::input::WithReadMultiLines;

use crate::direction::Direction;
use crate::line::Line;
use crate::routing_diagram::RoutingDiagram;

mod direction;
mod line;
mod routing_diagram;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let routing_diagram: RoutingDiagram = File::open(&args[1])
        .read_multi_lines(1)
        .next()
        .or_exit_with(1);

    let (encountered, steps) = travel(&routing_diagram);
    println!(
        "If we follow the routing diagram, we travel {} steps and find the letters {}.",
        steps, encountered
    );
}

fn travel(routing_diagram: &RoutingDiagram) -> (String, usize) {
    let mut answer = (String::new(), 0);
    if let Some(start) = routing_diagram.find_start() {
        let (mut x, mut y) = start;
        let mut direction: Direction = Direction::Down;
        loop {
            let next = match routing_diagram.at((x, y)) {
                Line::Horizontal | Line::Vertical => direction.step((x, y)),
                Line::Junction => match direction {
                    Direction::Right | Direction::Left => {
                        if routing_diagram.at((x, y - 1)).is_vertical() {
                            direction = Direction::Up;
                            (x, y - 1)
                        } else if routing_diagram.at((x, y + 1)).is_vertical() {
                            direction = Direction::Down;
                            (x, y + 1)
                        } else {
                            unreachable!()
                        }
                    }
                    Direction::Up | Direction::Down => {
                        if routing_diagram.at((x - 1, y)).is_horizontal() {
                            direction = Direction::Left;
                            (x - 1, y)
                        } else if routing_diagram.at((x + 1, y)).is_horizontal() {
                            direction = Direction::Right;
                            (x + 1, y)
                        } else {
                            unreachable!()
                        }
                    }
                },
                Line::Empty => break,
                Line::Letter(ch) => {
                    answer.0.push(ch);
                    direction.step((x, y))
                }
            };

            answer.1 += 1;
            x = next.0;
            y = next.1;
        }
    }
    answer
}

#[cfg(test)]
mod tests {
    use rdcl_aoc_helpers::input::WithAsMultilineRecords;

    use super::*;

    #[test]
    fn test_travel() {
        let test_input = vec![
            "     |          ",
            "     |  +--+    ",
            "     A  |  C    ",
            " F---|--|-E---+ ",
            "     |  |  |  D ",
            "     +B-+  +--+ ",
        ];
        let parsed = test_input.as_multiline_records::<RoutingDiagram>().unwrap();
        let routing_diagram = parsed.first().unwrap();

        assert_eq!(travel(routing_diagram), ("ABCDEF".to_string(), 38));
    }
}
