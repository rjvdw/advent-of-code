use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;
use rdcl_aoc_helpers::input::WithReadLines;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let line = File::open(&args[1])
        .read_lines::<String>(1)
        .next()
        .or_exit_with(1);

    let decompressed = decompress(&line);
    println!(
        "The length of the decompressed string is: {}",
        decompressed.len()
    );

    let ultimate_length = compute_ultimate_length(&line);
    println!("The ultimate length is: {}", ultimate_length);
}

enum ParsingMode {
    Plain,
    MarkerLength,
    MarkerRepeat,
    MarkerValue,
}

struct Marker {
    length: usize,
    repeat: usize,
}

impl Default for Marker {
    fn default() -> Self {
        Marker {
            length: 0,
            repeat: 0,
        }
    }
}

fn decompress(line: &str) -> String {
    let mut decompressed = String::new();
    let mut mode: ParsingMode = ParsingMode::Plain;
    let mut marker: Marker = Default::default();
    let mut value = String::new();

    for ch in line.chars() {
        match mode {
            ParsingMode::Plain => {
                if ch == '(' {
                    mode = ParsingMode::MarkerLength;
                    marker = Default::default();
                    value = String::new();
                } else {
                    decompressed.push(ch);
                }
            }
            ParsingMode::MarkerLength => {
                if ch == 'x' {
                    marker.length = value.parse().unwrap();
                    mode = ParsingMode::MarkerRepeat;
                    value = String::new();
                } else {
                    value.push(ch);
                }
            }
            ParsingMode::MarkerRepeat => {
                if ch == ')' {
                    marker.repeat = value.parse().unwrap();
                    mode = ParsingMode::MarkerValue;
                    value = String::new();
                } else {
                    value.push(ch);
                }
            }
            ParsingMode::MarkerValue => {
                value.push(ch);
                marker.length -= 1;
                if marker.length == 0 {
                    for _ in 0..marker.repeat {
                        decompressed.push_str(&value);
                    }
                    mode = ParsingMode::Plain;
                }
            }
        }
    }

    decompressed
}

fn compute_ultimate_length(line: &str) -> u64 {
    let mut length = 0;
    let mut mode: ParsingMode = ParsingMode::Plain;
    let mut marker: Marker = Default::default();
    let mut value = String::new();

    for ch in line.chars() {
        match mode {
            ParsingMode::Plain => {
                if ch == '(' {
                    mode = ParsingMode::MarkerLength;
                    marker = Default::default();
                    value = String::new();
                } else {
                    length += 1;
                }
            }
            ParsingMode::MarkerLength => {
                if ch == 'x' {
                    marker.length = value.parse().unwrap();
                    mode = ParsingMode::MarkerRepeat;
                    value = String::new();
                } else {
                    value.push(ch);
                }
            }
            ParsingMode::MarkerRepeat => {
                if ch == ')' {
                    marker.repeat = value.parse().unwrap();
                    mode = ParsingMode::MarkerValue;
                    value = String::new();
                } else {
                    value.push(ch);
                }
            }
            ParsingMode::MarkerValue => {
                value.push(ch);
                marker.length -= 1;
                if marker.length == 0 {
                    length += compute_ultimate_length(&value) * (marker.repeat as u64);
                    mode = ParsingMode::Plain;
                }
            }
        }
    }

    length
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decompress_1() {
        assert_eq!(decompress("ADVENT"), "ADVENT");
    }

    #[test]
    fn test_decompress_2() {
        assert_eq!(decompress("A(1x5)BC"), "ABBBBBC");
    }

    #[test]
    fn test_decompress_3() {
        assert_eq!(decompress("(3x3)XYZ"), "XYZXYZXYZ");
    }

    #[test]
    fn test_decompress_4() {
        assert_eq!(decompress("A(2x2)BCD(2x2)EFG"), "ABCBCDEFEFG");
    }

    #[test]
    fn test_decompress_5() {
        assert_eq!(decompress("(6x1)(1x3)A"), "(1x3)A");
    }

    #[test]
    fn test_decompress_6() {
        assert_eq!(decompress("X(8x2)(3x3)ABCY"), "X(3x3)ABC(3x3)ABCY");
    }

    #[test]
    fn test_compute_ultimate_length_1() {
        assert_eq!(compute_ultimate_length("(3x3)XYZ"), 9);
    }

    #[test]
    fn test_compute_ultimate_length_2() {
        assert_eq!(compute_ultimate_length("X(8x2)(3x3)ABCY"), 20);
    }

    #[test]
    fn test_compute_ultimate_length_3() {
        assert_eq!(
            compute_ultimate_length("(27x12)(20x12)(13x14)(7x10)(1x12)A"),
            241920
        );
    }

    #[test]
    fn test_compute_ultimate_length_4() {
        assert_eq!(
            compute_ultimate_length("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"),
            445
        );
    }
}
