use std::collections::HashMap;
use std::fs::File;
use std::thread;
use std::time::Duration;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;
use rdcl_aoc_helpers::input::WithReadLines;
use termion::{clear, color, cursor};

use scanner::Scanner;

fn main() {
    let args = get_args(
        &[
            "<input file>",
            "<delay>",
            "<speed during delay>",
            "<speed after delay>",
        ],
        1,
    );
    let scanners = File::open(&args[1]).read_lines(1).collect::<Vec<Scanner>>();
    let delay = args[2].parse::<u64>().or_exit_with(1);
    let speed_during_delay = args[3].parse::<usize>().or_exit_with(1);
    let speed_after_delay = args[4].parse::<usize>().or_exit_with(1);

    let visualisation_data = Scanner::get_visualisation_data(&scanners);
    let sleep_time_after_delay = Duration::from_millis(speed_after_delay as u64);
    let mut caught = (0, 0);

    if speed_during_delay > 0 {
        let sleep_time_during_delay = Duration::from_millis(speed_during_delay as u64);
        for t in 0..delay {
            visualise(&visualisation_data, t, u64::MAX, &mut caught);
            thread::sleep(sleep_time_during_delay);
        }
    }

    for t in delay..=delay + visualisation_data.1 {
        visualise(&visualisation_data, t, t - delay, &mut caught);
        thread::sleep(sleep_time_after_delay);
    }
}

fn visualise(
    (scanners, max_depth, max_range): &(HashMap<u64, u64>, u64, u64),
    time: u64,
    position: u64,
    (caught, severity): &mut (usize, u64),
) {
    print!("{}{}", clear::All, cursor::Goto(1, 1));
    println!("Picosecond {}:", time);

    for d in 0..=*max_depth {
        print!("{:2}  ", d);
    }
    println!();

    for range in 0..=*max_range {
        for depth in 0..=*max_depth {
            let mut color_inner = 'w';
            let mut color_outer = 'w';

            let (lp, rp) = if range == 0 && depth == position {
                color_outer = 'g';
                ('(', ')')
            } else if let Some(scanner_range) = scanners.get(&depth) {
                if range < *scanner_range {
                    ('[', ']')
                } else {
                    (' ', ' ')
                }
            } else if range == 0 {
                ('.', '.')
            } else {
                (' ', ' ')
            };

            let mp = if let Some(scanner_range) = scanners.get(&depth) {
                let scanner_range = *scanner_range;
                let scanner_period = (scanner_range - 1) * 2;
                let mut scanner_position = time % scanner_period;
                if scanner_position >= scanner_range {
                    scanner_position = scanner_range - (scanner_position - scanner_range) - 2;
                }

                if scanner_position == range {
                    color_inner = 'm';
                    if lp == '(' {
                        color_outer = 'r';
                        color_inner = 'r';
                        *caught += 1;
                        *severity += scanner_range * depth;
                    }
                    'S'
                } else {
                    ' '
                }
            } else if range == 0 {
                '.'
            } else {
                ' '
            };

            match (color_outer, color_inner) {
                ('r', 'r') => print!(
                    "{}{}{}{}{} ",
                    color::Fg(color::Red),
                    lp,
                    mp,
                    rp,
                    color::Fg(color::Reset)
                ),
                ('g', _) => print!(
                    "{}{}{}{}{} ",
                    color::Fg(color::Green),
                    lp,
                    mp,
                    rp,
                    color::Fg(color::Reset)
                ),
                (_, 'm') => print!(
                    "{}{}{}{}{} ",
                    lp,
                    color::Fg(color::LightMagenta),
                    mp,
                    color::Fg(color::Reset),
                    rp
                ),
                _ => print!("{}{}{} ", lp, mp, rp),
            }
        }
        println!()
    }
    println!("Caught {} times, severity: {}", caught, severity);
}
