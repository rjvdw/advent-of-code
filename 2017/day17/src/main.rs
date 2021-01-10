use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;

fn main() {
    let args = get_args(&["<step size>"], 1);
    let step_size = args[1].parse::<usize>().or_exit_with(1);

    println!(
        "After 2017 steps, the value after 2017 will be {}.",
        build_buffer(2017, step_size)[2017]
    );

    println!(
        "After fifty million steps, the value after 0 will be {}.",
        find_value_at_1(50_000_000, step_size)
    )
}

fn build_buffer(insert_count: usize, step_size: usize) -> Vec<usize> {
    let mut buffer = vec![0; insert_count + 1];
    let mut current = 0;

    for i in 1..=insert_count {
        for _ in 0..step_size {
            current = buffer[current];
        }
        let t = buffer[current];
        buffer[current] = i;
        buffer[i] = t;
        current = i;
    }

    buffer
}

fn find_value_at_1(insert_count: usize, step_size: usize) -> usize {
    let mut value = 0;
    let mut current = 0;
    for i in 1..=insert_count {
        current = (current + step_size) % i + 1;
        if current == 1 {
            value = i;
        }
    }
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_buffer() {
        assert_eq!(build_buffer(2017, 3)[2017], 638);
    }

    #[test]
    fn test_find_value_at_1() {
        assert_eq!(find_value_at_1(50_000_000, 3), 1_222_153);
    }
}
