use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::input::WithReadLines;

const ALPHABET_SIZE: usize = 26;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let box_ids = File::open(&args[1]).read_lines(1).collect::<Vec<String>>();

    println!("The checksum is {}.", compute_checksum(&box_ids));
    match find_prototype_fabric(&box_ids) {
        Some(common_letters) => println!(
            "The common letters between the boxes that contain the prototype fabric are: '{}'.",
            common_letters
        ),
        None => eprintln!("Could not find the prototype fabric."),
    }
}

fn compute_checksum(box_ids: &[String]) -> i32 {
    let mut counts = (0, 0);
    for box_id in box_ids {
        let (has2, has3) = check_frequencies(box_id);
        if has2 {
            counts.0 += 1;
        }
        if has3 {
            counts.1 += 1;
        }
    }
    counts.0 * counts.1
}

fn check_frequencies(box_id: &str) -> (bool, bool) {
    let mut frequencies = [0; ALPHABET_SIZE];
    for ch in box_id.chars() {
        frequencies[(ch as u8 - b'a') as usize] += 1;
    }
    (frequencies.contains(&2), frequencies.contains(&3))
}

fn find_prototype_fabric(box_ids: &[String]) -> Option<String> {
    for (idx, box_id_1) in box_ids.iter().enumerate() {
        'inner: for box_id_2 in box_ids.iter().skip(idx + 1) {
            let mut common_letters = String::new();
            let mut found_difference = false;
            for (ch1, ch2) in box_id_1.chars().zip(box_id_2.chars()) {
                if ch1 != ch2 {
                    if found_difference {
                        continue 'inner;
                    } else {
                        found_difference = true;
                    }
                } else {
                    common_letters.push(ch1);
                }
            }
            return Some(common_letters);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_frequencies_1() {
        assert_eq!(check_frequencies("abcdef"), (false, false));
    }

    #[test]
    fn test_check_frequencies_2() {
        assert_eq!(check_frequencies("bababc"), (true, true));
    }

    #[test]
    fn test_check_frequencies_3() {
        assert_eq!(check_frequencies("abbcde"), (true, false));
    }

    #[test]
    fn test_check_frequencies_4() {
        assert_eq!(check_frequencies("abcccd"), (false, true));
    }

    #[test]
    fn test_check_frequencies_5() {
        assert_eq!(check_frequencies("aabcdd"), (true, false));
    }

    #[test]
    fn test_check_frequencies_6() {
        assert_eq!(check_frequencies("abcdee"), (true, false));
    }

    #[test]
    fn test_check_frequencies_7() {
        assert_eq!(check_frequencies("ababab"), (false, true));
    }

    #[test]
    fn test_compute_checksum() {
        assert_eq!(
            compute_checksum(&[
                "abcdef".to_string(),
                "bababc".to_string(),
                "abbcde".to_string(),
                "abcccd".to_string(),
                "aabcdd".to_string(),
                "abcdee".to_string(),
                "ababab".to_string(),
            ]),
            12
        );
    }

    #[test]
    fn test_find_prototype_fabric() {
        assert_eq!(
            find_prototype_fabric(&[
                "abcde".to_string(),
                "fghij".to_string(),
                "klmno".to_string(),
                "pqrst".to_string(),
                "fguij".to_string(),
                "axcye".to_string(),
                "wvxyz".to_string(),
            ]),
            Some("fgij".to_string())
        );
    }
}
