use std::fmt;
use std::fs::File;

use json::{Error, JsonError, JsonValue};
use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;
use rdcl_aoc_helpers::input::WithReadLines;

fn main() {
    let args = get_args(&["<input file>", "<ignore string>"], 1);
    let lines = File::open(&args[1]).read_lines::<String>(1);
    let ignore_string = args[2].to_string();

    let mut counts = (0, 0);
    for line in lines {
        let r = count_numbers(&line, &ignore_string).or_exit_with(1);
        counts.0 += r.0;
        counts.1 += r.1;
    }
    println!("The sum of the numbers in the input file is: {}", counts.0);
    println!(
        "Ignoring objects containing the value \"{}\", the sum is: {}",
        ignore_string, counts.1
    );
}

fn count_numbers(json_str: &str, ignore: &str) -> Result<(i32, i32), ParseError> {
    let data = json::parse(json_str)?;
    Ok(count_numbers_in(&data, ignore))
}

fn count_numbers_in(data: &JsonValue, ignore: &str) -> (i32, i32) {
    if let Some(nr) = data.as_i32() {
        (nr, nr)
    } else {
        match data {
            JsonValue::Object(obj) => {
                let mut counts = (0, 0);
                let mut contains_ignore = false;
                for (_, value) in obj.iter() {
                    if value == ignore {
                        contains_ignore = true;
                    }
                    let r = count_numbers_in(value, ignore);
                    counts.0 += r.0;
                    counts.1 += r.1;
                }
                if contains_ignore {
                    counts.1 = 0;
                }
                counts
            }
            JsonValue::Array(arr) => arr
                .iter()
                .map(|jv| count_numbers_in(jv, ignore))
                .fold((0, 0), |acc, res| (acc.0 + res.0, acc.1 + res.1)),
            _ => (0, 0),
        }
    }
}

#[derive(Debug, PartialEq)]
struct ParseError(String);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<std::num::ParseIntError> for ParseError {
    fn from(err: std::num::ParseIntError) -> Self {
        ParseError(format!("{:?}", err))
    }
}

impl From<JsonError> for ParseError {
    fn from(err: Error) -> Self {
        ParseError(format!("Invalid JSON: {}", err))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_numbers_1() {
        assert_eq!(count_numbers(r#"[1,2,3]"#, "red"), Ok((6, 6)));
    }

    #[test]
    fn test_count_numbers_2() {
        assert_eq!(count_numbers(r#"{"a":2,"b":4}"#, "red"), Ok((6, 6)));
    }

    #[test]
    fn test_count_numbers_3() {
        assert_eq!(count_numbers(r#"[[[3]]]"#, "red"), Ok((3, 3)));
    }

    #[test]
    fn test_count_numbers_4() {
        assert_eq!(count_numbers(r#"{"a":{"b":4},"c":-1}"#, "red"), Ok((3, 3)));
    }

    #[test]
    fn test_count_numbers_5() {
        assert_eq!(count_numbers(r#"{"a":[-1,1]}"#, "red"), Ok((0, 0)));
    }

    #[test]
    fn test_count_numbers_6() {
        assert_eq!(count_numbers(r#"[-1,{"a":1}]"#, "red"), Ok((0, 0)));
    }

    #[test]
    fn test_count_numbers_7() {
        assert_eq!(count_numbers(r#"[]"#, "red"), Ok((0, 0)));
    }

    #[test]
    fn test_count_numbers_8() {
        assert_eq!(count_numbers(r#"{}"#, "red"), Ok((0, 0)));
    }

    #[test]
    fn test_count_numbers_9() {
        assert_eq!(
            count_numbers(r#"[1,{"c":"red","b":2},3]"#, "red"),
            Ok((6, 4))
        );
    }

    #[test]
    fn test_count_numbers_10() {
        assert_eq!(
            count_numbers(r#"{"d":"red","e":[1,2,3,4],"f":5}"#, "red"),
            Ok((15, 0))
        );
    }

    #[test]
    fn test_count_numbers_11() {
        assert_eq!(count_numbers(r#"[1,"red",5]"#, "red"), Ok((6, 6)));
    }
}
