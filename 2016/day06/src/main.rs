use std::convert::TryFrom;
use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::input::WithReadLines;

const ALPHABET_SIZE: usize = 26;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let lines = File::open(&args[1]).read_lines(1).collect::<Vec<String>>();

    let (message1, message2) = decode_message(&lines);
    println!("[Part 1] The message is '{}'.", message1);
    println!("[Part 2] The message is '{}'.", message2);
}

fn decode_message(lines: &[String]) -> (String, String) {
    let mut frequencies: Vec<[u32; ALPHABET_SIZE]> = Vec::new();

    for line in lines {
        frequencies.resize_with(line.len(), || [0; ALPHABET_SIZE]);
        for (idx, ch) in line.chars().enumerate() {
            let ch_idx = ((ch as u8) - b'a') as usize;
            frequencies[idx][ch_idx] += 1;
        }
    }

    let mut decoded1 = String::new();
    let mut decoded2 = String::new();

    for position in frequencies {
        let max = *position.iter().max().unwrap();
        let idx = position.iter().position(|f| *f == max).unwrap();
        let ch = (u8::try_from(idx).unwrap() + b'a') as char;
        decoded1.push(ch);

        let min = *position.iter().filter(|&f| *f != 0).min().unwrap();
        let idx = position.iter().position(|f| *f == min).unwrap();
        let ch = (u8::try_from(idx).unwrap() + b'a') as char;
        decoded2.push(ch);
    }

    (decoded1, decoded2)
}

#[cfg(test)]
mod tests {
    use rdcl_aoc_helpers::input::WithAsRecords;

    use super::*;

    #[test]
    fn test_decode_message() {
        let lines = vec![
            "eedadn", "drvtee", "eandsr", "raavrd", "atevrs", "tsrnev", "sdttsa", "rasrtv",
            "nssdts", "ntnada", "svetve", "tesnvt", "vntsnd", "vrdear", "dvrsen", "enarar",
        ]
        .as_records::<String>()
        .unwrap();

        assert_eq!(
            decode_message(&lines),
            ("easter".to_string(), "advent".to_string())
        );
    }
}
