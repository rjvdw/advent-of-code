extern crate rdcl_aoc_helpers;

use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::input::WithReadLines;

use crate::display::{count_segments, Display};

mod display;

/// https://adventofcode.com/2021/day/8
fn main() {
    let args = get_args(&["<input file>"], 1);

    let displays = File::open(&args[1]).read_lines(1).collect::<Vec<Display>>();

    println!(
        "There are {} easy digits in the output",
        count_easy_digits(&displays)
    );

    println!(
        "The sum of all the displays is {}.",
        decode_displays(&displays)
    )
}

fn count_easy_digits(displays: &[Display]) -> usize {
    displays
        .iter()
        .map(|display| display.output)
        .flat_map(|output| {
            output
                .iter()
                .map(|&digit| count_segments(digit))
                .collect::<Vec<usize>>()
        })
        .filter(|&count| count == 2 || count == 3 || count == 4 || count == 7)
        .count()
}

fn decode_displays(displays: &[Display]) -> u32 {
    displays.iter().map(|&display| decode(display)).sum()
}

fn decode(display: Display) -> u32 {
    // count the segments per digit once
    let mut digits_with_counts = [(0usize, 0u8); 10];
    for (idx, &digit) in display.digits.iter().enumerate() {
        digits_with_counts[idx] = (count_segments(digit), digit);
    }

    // determine which digit corresponds with each input
    let mut digits = [0u8; 10];
    for &(count, digit) in &digits_with_counts {
        match count {
            2 => digits[1] = digit,
            3 => digits[7] = digit,
            4 => digits[4] = digit,
            7 => digits[8] = digit,
            _ => {}
        }
    }
    for &(count, digit) in &digits_with_counts {
        match count {
            6 if (digit | digits[4] == digit) => digits[9] = digit,
            6 if (digit | digits[1] == digit) => digits[0] = digit,
            6 => digits[6] = digit,
            _ => {}
        }
    }
    for &(count, digit) in &digits_with_counts {
        match count {
            5 if (digit | digits[6] == digits[6]) => digits[5] = digit,
            5 if (digit | digits[9] == digits[9]) => digits[3] = digit,
            5 => digits[2] = digit,
            _ => {}
        }
    }

    // determine the mapping
    let mut mapping = [0u8; 7];
    mapping[0] = digits[1] ^ digits[7]; // compare 1 and 7 to find a
    mapping[2] = digits[5] ^ digits[9]; // compare 5 and 9 to find c
    mapping[3] = digits[0] ^ digits[8]; // compare 0 and 8 to find d
    mapping[4] = digits[8] ^ digits[9]; // compare 8 and 9 to find e
    mapping[1] = (digits[8] - mapping[4]) ^ digits[3]; // compare (8 - e) and 3 to find b
    mapping[5] = (digits[8] - mapping[1]) ^ digits[2]; // compare (8 - b) and 2 to find f
    mapping[6] = (digits[4] | mapping[0]) ^ digits[9]; // compare (4 + a) and 9 to find g

    // finally we can retrieve the output value of this display
    display.get_output(mapping).unwrap()
}

#[cfg(test)]
mod tests {
    use rdcl_aoc_helpers::input::WithAsRecords;

    use super::*;

    #[test]
    fn test_count_easy_digits() {
        let displays = vec![
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
            "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
            "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
            "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
            "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
            "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
            "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
            "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
            "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
            "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
        ].as_records::<Display>().unwrap();

        assert_eq!(count_easy_digits(&displays), 26);
    }

    #[test]
    fn test_decode_displays() {
        let displays = vec![
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
            "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
            "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
            "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
            "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
            "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
            "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
            "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
            "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
            "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
        ].as_records::<Display>().unwrap();

        assert_eq!(decode_displays(&displays), 61229);
    }

    #[test]
    fn test_decode() {
        let displays = vec![
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
            "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
            "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
            "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
            "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
            "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
            "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
            "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
            "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
            "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
        ]
            .iter()
            .map(|line| line.parse::<Display>().unwrap())
            .map(decode)
            .collect::<Vec<u32>>();

        assert_eq!(
            displays,
            vec![8394, 9781, 1197, 9361, 4873, 8418, 4548, 1625, 8717, 4315],
        );
    }
}
