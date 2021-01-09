use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;
use rdcl_aoc_helpers::input::WithReadLines;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let stream: String = File::open(&args[1]).read_lines(1).next().or_exit_with(1);

    let (score, garbage) = process_stream(&stream);
    println!("The total score is {}.", score);
    println!("There are {} characters in the garbage.", garbage);
}

fn process_stream(stream: &str) -> (usize, usize) {
    let mut score: usize = 0;
    let mut depth: usize = 0;
    let mut parsing = Parsing::Stream;
    let mut garbage: usize = 0;

    let mut chars = stream.chars();
    while let Some(ch) = chars.next() {
        match ch {
            '!' => {
                chars.next();
            }
            '<' => {
                if parsing == Parsing::Stream {
                    parsing = Parsing::Garbage;
                } else {
                    garbage += 1;
                }
            }
            '>' => {
                parsing = Parsing::Stream;
            }
            '{' => {
                if parsing == Parsing::Stream {
                    depth += 1;
                } else {
                    garbage += 1;
                }
            }
            '}' => {
                if parsing == Parsing::Stream {
                    score += depth;
                    depth -= 1;
                } else {
                    garbage += 1;
                }
            }
            ',' => {
                if parsing == Parsing::Garbage {
                    garbage += 1;
                }
            }
            _ => {
                if parsing == Parsing::Stream {
                    panic!("Invalid character encountered.");
                } else {
                    garbage += 1;
                }
            }
        }
    }

    assert_eq!(depth, 0);

    (score, garbage)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Parsing {
    Stream,
    Garbage,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_stream() {
        assert_eq!(process_stream("{}").0, 1);
        assert_eq!(process_stream("{{{}}}").0, 6);
        assert_eq!(process_stream("{{},{}}").0, 5);
        assert_eq!(process_stream("{{{},{},{{}}}}").0, 16);
        assert_eq!(process_stream("{<a>,<a>,<a>,<a>}").0, 1);
        assert_eq!(process_stream("{{<ab>},{<ab>},{<ab>},{<ab>}}").0, 9);
        assert_eq!(process_stream("{{<!!>},{<!!>},{<!!>},{<!!>}}").0, 9);
        assert_eq!(process_stream("{{<a!>},{<a!>},{<a!>},{<ab>}}").0, 3);

        assert_eq!(process_stream("<>").1, 0);
        assert_eq!(process_stream("<random characters>").1, 17);
        assert_eq!(process_stream("<<<<>").1, 3);
        assert_eq!(process_stream("<{!>}>").1, 2);
        assert_eq!(process_stream("<!!>").1, 0);
        assert_eq!(process_stream("<!!!>>").1, 0);
        assert_eq!(process_stream(r#"<{o"i!a,<{i<a>"#).1, 10);
    }
}
