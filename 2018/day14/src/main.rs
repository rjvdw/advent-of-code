use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;

const RECIPE_SCORING_SIZE: usize = 10;

fn main() {
    let args = get_args(&["<nr steps>", "<score 1>", "<score 2>"], 1);
    let sequence = &args[1];
    let elf1_score = args[2].parse::<u8>().or_exit_with(1);
    let elf2_score = args[3].parse::<u8>().or_exit_with(1);

    let scores_after = get_score_after(sequence, elf1_score, elf2_score, RECIPE_SCORING_SIZE);
    println!("The score after {} steps is {}.", sequence, scores_after);

    let nr_recipes = first_score_that_satisfies(sequence, elf1_score, elf2_score);
    println!(
        "There are {} recipes to the left of the score sequence '{}'.",
        nr_recipes, sequence
    )
}

fn get_score_after(sequence: &str, elf1_score: u8, elf2_score: u8, scoring_size: usize) -> String {
    let nr_steps = sequence.parse::<usize>().unwrap();
    let mut scores = vec![elf1_score, elf2_score];
    let mut elf1 = 0;
    let mut elf2 = 1;
    while scores.len() < nr_steps + scoring_size {
        let next = improve_recipes(&mut scores, elf1, elf2);
        elf1 = next.0;
        elf2 = next.1;
    }

    scores
        .iter()
        .skip(nr_steps)
        .take(scoring_size)
        .map(|&b| (b + b'0') as char)
        .fold(String::new(), |mut acc, ch| {
            acc.push(ch);
            acc
        })
}

fn first_score_that_satisfies(sequence: &str, elf1_score: u8, elf2_score: u8) -> usize {
    let mut scores = vec![elf1_score, elf2_score];
    let mut elf1 = 0;
    let mut elf2 = 1;
    let needle = sequence.bytes().map(|b| b - b'0').collect::<Vec<u8>>();
    let mut check_from = 0;
    loop {
        let old_len = scores.len();
        let next = improve_recipes(&mut scores, elf1, elf2);
        elf1 = next.0;
        elf2 = next.1;

        if old_len >= needle.len() {
            let next_check_from = check_from + scores.len() - old_len;
            for start in check_from..next_check_from {
                let mut is_match = true;
                for (idx, &ch) in needle.iter().enumerate() {
                    if ch != scores[idx + start] {
                        is_match = false;
                        break;
                    }
                }
                if is_match {
                    return start;
                }
            }
            check_from = next_check_from;
        }
    }
}

fn improve_recipes(scores: &mut Vec<u8>, elf1: usize, elf2: usize) -> (usize, usize) {
    let combined = scores[elf1] + scores[elf2];
    for digit in combined.to_string().chars().map(|ch| (ch as u8) - b'0') {
        scores.push(digit);
    }
    (next_recipe(elf1, scores), next_recipe(elf2, scores))
}

fn next_recipe(elf: usize, scores: &[u8]) -> usize {
    (elf + 1 + scores[elf] as usize) % scores.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_improve_recipes() {
        let mut scores = vec![3, 7];
        assert_eq!(improve_recipes(&mut scores, 0, 1), (0, 1));
        assert_eq!(scores, vec![3, 7, 1, 0]);
    }

    #[test]
    fn test_get_score_after() {
        assert_eq!(get_score_after("9", 3, 7, 10), "5158916779");
        assert_eq!(get_score_after("5", 3, 7, 10), "0124515891");
        assert_eq!(get_score_after("18", 3, 7, 10), "9251071085");
        assert_eq!(get_score_after("2018", 3, 7, 10), "5941429882");
    }

    #[test]
    fn test_first_score_that_satisfies() {
        assert_eq!(first_score_that_satisfies("51589", 3, 7), 9);
        assert_eq!(first_score_that_satisfies("01245", 3, 7), 5);
        assert_eq!(first_score_that_satisfies("92510", 3, 7), 18);
        assert_eq!(first_score_that_satisfies("59414", 3, 7), 2018);
    }
}
