use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;

fn main() {
    let args = get_args(
        &[
            "<starting number>",
            "<nr steps part 1>",
            "<nr steps part 2>",
        ],
        1,
    );
    let mut number = args[1].to_string();
    let part_1_steps = args[2].parse::<usize>().or_exit_with(1);
    let part_2_steps = args[3].parse::<usize>().or_exit_with(1);

    let min = part_1_steps.min(part_2_steps);
    let max = part_1_steps.max(part_2_steps);

    for _ in 0..min {
        number = step(number).or_exit_with(1);
    }
    println!("After {} steps, the length is: {}", min, number.len());

    for _ in min..max {
        number = step(number).or_exit_with(1);
    }
    println!("After {} steps, the length is: {}", max, number.len());
}

fn step(number: String) -> Option<String> {
    let mut result = String::new();
    let mut chars = number.chars();
    let mut count = 1;
    let mut symbol = chars.next()?;

    for char in chars {
        if char == symbol {
            count += 1;
        } else {
            result.push_str(count.to_string().as_str());
            result.push(symbol);
            count = 1;
            symbol = char;
        }
    }

    result.push_str(count.to_string().as_str());
    result.push(symbol);

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(step("1".to_string()), Some("11".to_string()));
    }

    #[test]
    fn test_2() {
        assert_eq!(step("11".to_string()), Some("21".to_string()));
    }

    #[test]
    fn test_3() {
        assert_eq!(step("21".to_string()), Some("1211".to_string()));
    }

    #[test]
    fn test_4() {
        assert_eq!(step("1211".to_string()), Some("111221".to_string()));
    }

    #[test]
    fn test_5() {
        assert_eq!(step("111221".to_string()), Some("312211".to_string()));
    }
}
