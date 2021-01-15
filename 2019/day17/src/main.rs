use std::fs;
use std::fs::File;

use grid::Grid;
use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::direction::Direction;
use rdcl_aoc_helpers::error::WithOrExit;

use shared::intcode;
use shared::intcode::parse::parse_input;

fn main() {
    let args = get_args(&["<input file>", "<solution>"], 1);
    let file = File::open(&args[1]).or_exit_with(1);
    let program = parse_input(file).or_exit_with(1);
    let solution = fs::read_to_string(&args[2]).or_exit_with(1);

    let map = get_map(program.clone());
    // render_map(&map);
    println!(
        "The sum of the alignment parameters is {}.",
        get_alignment_parameters(&map).iter().sum::<usize>()
    );
    println!(
        "The vacuum robot has collected {} units of dust.",
        follow_instructions(program, &solution)
    )
}

fn follow_instructions(mut program: intcode::Program, solution: &str) -> i64 {
    program[0] = 2;
    for &ch in solution.as_bytes() {
        program.send_message(ch as i64);
    }
    program.send_message(b'n' as i64);
    program.send_message(b'\n' as i64);
    program.run();

    let mut dump = program.output_dump();
    dump.pop().unwrap()
    // let collected = dump.pop().unwrap();
    // let output: Vec<u8> = dump.iter().map(|&c| c as u8).collect();
    // println!("{}", std::str::from_utf8(&output).unwrap());
    //
    // collected
}

fn get_alignment_parameters(map: &Grid<Space>) -> Vec<usize> {
    let mut intersections = vec![];
    for y in 1..map.rows() - 1 {
        for x in 1..map.cols() - 1 {
            let is_intersection = map[y][x].is_scaffold()
                && map[y - 1][x].is_scaffold()
                && map[y + 1][x].is_scaffold()
                && map[y][x - 1].is_scaffold()
                && map[y][x + 1].is_scaffold();

            if is_intersection {
                intersections.push(x * y);
            }
        }
    }
    intersections
}

#[allow(dead_code)]
fn render_map(map: &Grid<Space>) {
    for y in 0..map.rows() {
        for x in 0..map.cols() {
            match map[y][x] {
                Space::Scaffold => print!("#"),
                Space::Open => print!("."),
                Space::Robot(Direction::Up) => print!("^"),
                Space::Robot(Direction::Down) => print!("v"),
                Space::Robot(Direction::Left) => print!("<"),
                Space::Robot(Direction::Right) => print!(">"),
                Space::Tumbling => print!("X"),
            }
        }
        println!();
    }
}

fn get_map(mut program: intcode::Program) -> Grid<Space> {
    let mut spaces = vec![];
    let mut width = 0;
    program.run();

    while let Some(ch) = program.receive_message().map(|v| (v as u8) as char) {
        match ch {
            '#' => {
                spaces.push(Space::Scaffold);
            }
            '.' => {
                spaces.push(Space::Open);
            }
            '\n' => {
                if width == 0 {
                    width = spaces.len();
                }
            }
            '^' => {
                spaces.push(Space::Robot(Direction::Up));
            }
            '>' => {
                spaces.push(Space::Robot(Direction::Right));
            }
            'v' => {
                spaces.push(Space::Robot(Direction::Down));
            }
            '<' => {
                spaces.push(Space::Robot(Direction::Left));
            }
            'X' => {
                spaces.push(Space::Tumbling);
            }
            x => unreachable!("Unexpected value: {:?}", x),
        }
    }
    Grid::from_vec(spaces, width)
}

#[derive(Debug, Copy, Clone)]
enum Space {
    Scaffold,
    Open,
    Robot(Direction),
    Tumbling,
}

impl Space {
    fn is_scaffold(&self) -> bool {
        matches!(self, Space::Scaffold) || matches!(self, Space::Robot(_))
    }

    // fn is_open(&self) -> bool {
    //     matches!(self, Space::Open) || matches!(self, Space::Tumbling)
    // }
}
