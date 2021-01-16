use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;

use shared::intcode::parse::parse_input;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let file = File::open(&args[1]).or_exit_with(1);
    let program = parse_input(file).or_exit_with(1);
    let test = move |x, y| {
        let mut program = program.clone();
        program.send_message(x);
        program.send_message(y);
        program.run();
        match program.receive_message() {
            Some(0) => false,
            Some(1) => true,
            o => unreachable!("Unexpected output for ({}, {}): {:?}", x, y, o),
        }
    };

    let count = map_tractor_beam(&test, 0..50, 0..50, false, None);
    println!(
        "A total of {} points are affected by the tractor beam.",
        count
    );

    let (x, y) = find_minimum_distance(&test, 100, 100);
    // let ship = Ship {
    //     position: (x, y),
    //     width: 100,
    //     height: 100,
    // };
    // map_tractor_beam(&test, 850..1125, 1200..1400, true, Some(&ship));

    println!(
        "Place the ship at ({}, {}) to completely fit inside the tractor beam. Your answer is {}.",
        x,
        y,
        x * 10_000 + y
    );
}

struct Ship {
    position: (i64, i64),
    width: i64,
    height: i64,
}

impl Ship {
    fn contains(&self, x: i64, y: i64) -> bool {
        x >= self.position.0
            && y >= self.position.1
            && x < self.position.0 + self.width
            && y < self.position.1 + self.height
    }
}

fn map_tractor_beam<T, X, Y>(
    test: &T,
    x_range: X,
    y_range: Y,
    print: bool,
    ship: Option<&Ship>,
) -> usize
where
    T: Fn(i64, i64) -> bool,
    X: Iterator<Item = i64> + Clone,
    Y: Iterator<Item = i64>,
{
    let mut count = 0;
    for y in y_range {
        for x in x_range.clone() {
            if test(x, y) {
                count += 1;
                if print {
                    if let Some(ship) = ship {
                        if ship.contains(x, y) {
                            print!("O");
                        } else {
                            print!("#");
                        }
                    } else {
                        print!("#");
                    }
                }
            } else if print {
                if let Some(ship) = ship {
                    if ship.contains(x, y) {
                        print!("X");
                    } else {
                        print!(".");
                    }
                } else {
                    print!(".");
                }
            }
        }
        if print {
            println!();
        }
    }
    count
}

fn find_minimum_distance<T: Fn(i64, i64) -> bool>(test: &T, width: i64, height: i64) -> (i64, i64) {
    'row: for y in height.. {
        for x in 0.. {
            if test(x, y) {
                // println!("({}, {})", x, y);
                if test(x + width - 1, y - height + 1) {
                    return (x, y - height + 1);
                }
                continue 'row;
            }
        }
    }
    unreachable!()
}
