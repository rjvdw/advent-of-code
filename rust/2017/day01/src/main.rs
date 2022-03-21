use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;
use rdcl_aoc_helpers::input::WithReadLines;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let captcha = File::open(&args[1])
        .read_lines::<String>(1)
        .next()
        .or_exit_with(1);

    println!(
        "The solution to the captcha is {:?}.",
        solve_captcha(&captcha)
    );
}

fn solve_captcha(captcha: &str) -> (usize, usize) {
    let mut solution = (0, 0);
    let chars: Vec<usize> = captcha
        .chars()
        .map(|ch| (ch as u8 - b'0') as usize)
        .collect();
    let n = chars.len();

    for (idx, &el) in chars.iter().enumerate() {
        let idx1 = idx.checked_sub(1).unwrap_or(n - 1);
        let idx2 = (idx + n - n / 2) % n;

        if el == chars[idx1] {
            solution.0 += el;
        }

        if el == chars[idx2] {
            solution.1 += el;
        }
    }

    solution
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_captcha_part_1() {
        assert_eq!(solve_captcha("1122").0, 3);
        assert_eq!(solve_captcha("1111").0, 4);
        assert_eq!(solve_captcha("1234").0, 0);
        assert_eq!(solve_captcha("91212129").0, 9);
    }

    #[test]
    fn test_solve_captcha_part_2() {
        assert_eq!(solve_captcha("1212").1, 6);
        assert_eq!(solve_captcha("1221").1, 0);
        assert_eq!(solve_captcha("123425").1, 4);
        assert_eq!(solve_captcha("123123").1, 12);
        assert_eq!(solve_captcha("12131415").1, 4);
    }
}
