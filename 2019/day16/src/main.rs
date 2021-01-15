use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;
use rdcl_aoc_helpers::input::WithReadLines;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let input_string: String = File::open(&args[1]).read_lines(1).next().or_exit_with(1);

    let signal = fft(to_signal(&input_string, 1), vec![0, 1, 0, -1], 100);
    println!(
        "After 100 phases of FFT, the first eight digits are {}.",
        get_embedded_message(signal, 0)
    );

    let message = fft_with_shortcut(
        input_string[0..7].parse().or_exit_with(1),
        to_signal(&input_string, 10_000),
        100,
    );
    println!(
        "The message that is embedded in the real signal is {}.",
        message
    );
}

fn to_signal(s: &str, repeats: usize) -> Vec<u8> {
    let mut signal = Vec::with_capacity(s.len() * repeats);
    for _ in 0..repeats {
        signal.extend(s.bytes().map(|b| b - b'0'))
    }
    signal
}

fn get_embedded_message(signal: Vec<u8>, offset: usize) -> String {
    signal[offset..offset + 8]
        .iter()
        .map(|&d| (d + b'0') as char)
        .fold(String::new(), |mut acc, ch| {
            acc.push(ch);
            acc
        })
}

fn fft(mut signal: Vec<u8>, base_pattern: Vec<i32>, phases: usize) -> Vec<u8> {
    for _ in 1..=phases {
        let mut next_signal = Vec::with_capacity(signal.len());
        for n in 1..=signal.len() {
            let mut sum = 0;
            let mut sig_idx = 0;
            let mut pat_idx = 0;
            let mut repeater = 1;

            while sig_idx < signal.len() {
                if repeater == n || base_pattern[pat_idx] == 0 {
                    sig_idx += n - repeater;
                    repeater = 0;
                    pat_idx += 1;
                    if pat_idx == base_pattern.len() {
                        pat_idx = 0;
                    }
                } else {
                    sum += (signal[sig_idx] as i32) * base_pattern[pat_idx];
                    repeater += 1;
                    sig_idx += 1;
                }
            }

            next_signal.push((sum.abs() % 10) as u8);
        }
        signal = next_signal;
    }
    signal
}

fn fft_with_shortcut(offset: usize, mut signal: Vec<u8>, phases: usize) -> String {
    let mut sums = vec![0; signal.len() + 1];
    for _ in 1..=phases {
        for (idx, &s) in signal.iter().enumerate() {
            sums[idx + 1] = sums[idx] + (s as i32);
        }
        let last = *sums.last().unwrap();
        for (idx, s) in signal.iter_mut().enumerate() {
            *s = ((last - sums[idx]) % 10) as u8;
        }
    }
    get_embedded_message(signal, offset)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fft() {
        assert_eq!(
            fft(to_signal("12345678", 1), vec![0, 1, 0, -1], 1),
            to_signal("48226158", 1)
        );
        assert_eq!(
            fft(to_signal("12345678", 1), vec![0, 1, 0, -1], 2),
            to_signal("34040438", 1)
        );
        assert_eq!(
            fft(to_signal("12345678", 1), vec![0, 1, 0, -1], 3),
            to_signal("03415518", 1)
        );
        assert_eq!(
            fft(to_signal("12345678", 1), vec![0, 1, 0, -1], 4),
            to_signal("01029498", 1)
        );
    }

    #[test]
    fn test_fft_and_get_embedded_message_at_0() {
        let signal = fft(
            to_signal("80871224585914546619083218645595", 1),
            vec![0, 1, 0, -1],
            100,
        );
        assert_eq!(get_embedded_message(signal, 0), "24176176");

        let signal = fft(
            to_signal("19617804207202209144916044189917", 1),
            vec![0, 1, 0, -1],
            100,
        );
        assert_eq!(get_embedded_message(signal, 0), "73745418");

        let signal = fft(
            to_signal("69317163492948606335995924319873", 1),
            vec![0, 1, 0, -1],
            100,
        );
        assert_eq!(get_embedded_message(signal, 0), "52432133");
    }

    #[test]
    fn test_shortcut() {
        let input_string = "80871224585914546619083218645595";
        let signal = fft(to_signal(input_string, 1), vec![0, 1, 0, -1], 2);
        println!("{:?}", signal);
        let expected = get_embedded_message(signal, 16);

        let actual = fft_with_shortcut(16, to_signal(&input_string, 1), 2);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_fft_with_shortcut_1() {
        let input_string = "03036732577212944063491565474664";
        let message = fft_with_shortcut(
            input_string[0..7].parse().or_exit_with(1),
            to_signal(&input_string, 10_000),
            100,
        );
        assert_eq!(message, "84462026");
    }

    #[test]
    fn test_fft_with_shortcut_2() {
        let input_string = "02935109699940807407585447034323";
        let message = fft_with_shortcut(
            input_string[0..7].parse().or_exit_with(1),
            to_signal(&input_string, 10_000),
            100,
        );
        assert_eq!(message, "78725270");
    }

    #[test]
    fn test_fft_with_shortcut_3() {
        let input_string = "03081770884921959731165446850517";
        let message = fft_with_shortcut(
            input_string[0..7].parse().or_exit_with(1),
            to_signal(&input_string, 10_000),
            100,
        );
        assert_eq!(message, "53553731");
    }
}
