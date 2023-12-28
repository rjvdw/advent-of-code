use std::path::PathBuf;

use clap::Parser;
use rdcl_aoc_core::input::InputReader;

#[derive(Parser, Debug)]
#[clap(about = "The solution for advent of code 2023, day 20")]
struct Args {
    /// The file which contains the puzzle input.
    input: PathBuf,
}

/// Take the input and convert it into a format that can be imported
/// into <https://csacademy.com/app/graph_editor/>.
fn main() {
    let args: Args = Args::parse();
    for line in InputReader::from(args.input).read_lines() {
        let i = line.find(' ').expect("invalid input");
        let from = if line.starts_with('%') || line.starts_with('&') {
            &line[1..i]
        } else {
            &line[..i]
        };

        let i = i + 1 + line[i + 1..].find(' ').expect("invalid input");
        for to in line[i + 1..].split(", ") {
            println!("{from} {to}");
        }
    }
}
