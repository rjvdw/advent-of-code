//! The solution for [advent of code 2022, day 10](https://adventofcode.com/2022/day/10)

use std::path::PathBuf;

use clap::Parser;

use rdcl_aoc_core::err_parse_error;
use rdcl_aoc_core::error::ParseError;
use rdcl_aoc_core::input::InputReader;

use crate::output::Output;

mod ocr;
mod output;

#[derive(Parser, Debug)]
#[clap(about = "The solution for advent of code 2022, day 10")]
struct Args {
    /// The file which contains the puzzle input.
    input: PathBuf,

    /// The interval at which the signal strength is evaluated.
    #[clap(short, long, value_parser, default_value_t = 40)]
    interval: i64,

    /// The first cycle at which the signal strength is evaluated.
    #[clap(short, long, value_parser, default_value_t = 20)]
    offset: i64,

    /// Should the output be colorized?
    #[clap(short, long, value_parser, default_value_t = false)]
    color: bool,
}

fn main() {
    let args: Args = Args::parse();
    let input = InputReader::from(args.input);

    match run(input.read_lines(), args.interval, args.offset, args.color) {
        Ok((signal_strengths, output, text)) => {
            println!(
                "The sum of the signal strengths is {}",
                signal_strengths.iter().sum::<i64>()
            );

            println!("The display now looks like this:");
            print!("{}", output);
            println!("The text on the display is {}", text);
        }
        Err(e) => {
            eprintln!("The program failed: {}", e);
        }
    }
}

fn run<T>(
    input: T,
    interval: i64,
    offset: i64,
    use_color: bool,
) -> Result<(Vec<i64>, String, String), ParseError>
where
    T: Iterator<Item = String>,
{
    let mut signal_strengths = vec![];
    let mut cycle = 0;
    let mut register = 1;
    let mut output = Output::new();

    let mut tick = |register: i64| {
        let x = cycle % interval + 1;
        cycle += 1;
        output.write(register.abs_diff(x - 1) <= 1);

        if x == interval {
            output.next_line();
        }

        if x == offset {
            signal_strengths.push(cycle * register);
        };
    };

    for line in input {
        if line.eq("noop") {
            tick(register);
        } else if let Some(v) = line.strip_prefix("addx ") {
            tick(register);
            tick(register);
            register += v.parse::<i64>()?;
        } else {
            return err_parse_error!("Invalid input: {}", line);
        }
    }

    Ok((signal_strengths, output.read(use_color), output.ocr()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> impl Iterator<Item = String> {
        InputReader::from("./src/day10/test.txt").read_lines()
    }

    #[test]
    fn test_run() {
        let expected_signal_strength = vec![420, 1140, 1800, 2940, 2880, 3960];
        let expected_output = "\
            ##..##..##..##..##..##..##..##..##..##..\n\
            ###...###...###...###...###...###...###.\n\
            ####....####....####....####....####....\n\
            #####.....#####.....#####.....#####.....\n\
            ######......######......######......####\n\
            #######.......#######.......#######.....\n\
        "
        .to_string();

        assert_eq!(
            run(test_data(), 40, 20, false).unwrap(),
            (
                expected_signal_strength,
                expected_output,
                "????????".to_string()
            )
        );
    }
}
