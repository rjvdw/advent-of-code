use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::input::{MappedLines, WithReadLines};
use std::fs::File;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let lines = File::open(&args[1]).read_lines::<String>(1);

    let (before_encoding, after_encoding) = process_input(lines);

    println!(
        "The difference in length between code and in memory representation before encoding is: {}",
        before_encoding
    );
    println!(
        "The difference in length between code and in memory representation after encoding is: {}",
        after_encoding
    );
}

fn process_input(lines: MappedLines<String, File>) -> (i32, i32) {
    let mut count = (0, 0);
    for line in lines {
        let code_length = line.len() as i32;
        let actual_length = determine_actual_length(&line);
        let encoded_length = determine_encoded_length(&line);
        count.0 += code_length - actual_length;
        count.1 += encoded_length - code_length;
    }
    count
}

fn determine_actual_length(line: &str) -> i32 {
    let mut length = 0;
    let mut chars = line.chars();
    while let Some(ch) = chars.next() {
        length += match ch {
            '\\' => match chars.next() {
                Some('\\') | Some('"') => 1,
                Some('x') => {
                    // discard the next two characters
                    chars.next().unwrap();
                    chars.next().unwrap();
                    1
                }
                _ => panic!("Failed to process line: {}", line),
            },
            _ => 1,
        };
    }
    length - 2
}

fn determine_encoded_length(line: &str) -> i32 {
    let mut length = line.len() as i32;
    for ch in line.chars() {
        if ch == '\\' || ch == '"' {
            length += 1;
        }
    }
    length + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_actual_length_1() {
        assert_eq!(determine_actual_length(r#""""#), 0);
    }

    #[test]
    fn test_determine_actual_length_2() {
        assert_eq!(determine_actual_length(r#""abc""#), 3);
    }

    #[test]
    fn test_determine_actual_length_3() {
        assert_eq!(determine_actual_length(r#""aaa\"aaa""#), 7);
    }

    #[test]
    fn test_determine_actual_length_4() {
        assert_eq!(determine_actual_length(r#""\x27""#), 1);
    }

    #[test]
    fn test_determine_encoded_length_1() {
        assert_eq!(determine_encoded_length(r#""""#), 6);
    }

    #[test]
    fn test_determine_encoded_length_2() {
        assert_eq!(determine_encoded_length(r#""abc""#), 9);
    }

    #[test]
    fn test_determine_encoded_length_3() {
        assert_eq!(determine_encoded_length(r#""aaa\"aaa""#), 16);
    }

    #[test]
    fn test_determine_encoded_length_4() {
        assert_eq!(determine_encoded_length(r#""\x27""#), 11);
    }
}
