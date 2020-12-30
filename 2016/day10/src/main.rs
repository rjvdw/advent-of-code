use std::collections::HashMap;
use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;
use rdcl_aoc_helpers::input::WithReadLines;

use crate::io::{Input, Output};

mod io;

fn main() {
    let args = get_args(&["<input file>", "<v1>", "<v2>"], 1);
    let instructions = File::open(&args[1]).read_lines(1).collect::<Vec<Input>>();
    let v1 = args[2].parse::<usize>().or_exit_with(1);
    let v2 = args[3].parse::<usize>().or_exit_with(1);

    let (bot, outputs) = evaluate(&instructions, v1, v2);
    match bot {
        Some(bot) => println!("Bot {} first compared {} and {}.", bot, v1, v2),
        None => eprintln!("No bot ever compares {} and {}.", v1, v2),
    }
    let product = outputs.get(&0).unwrap_or(&0)
        * outputs.get(&1).unwrap_or(&0)
        * outputs.get(&2).unwrap_or(&0);
    println!("The product of outputs 0, 1, and 2 is: {}.", product);
}

fn evaluate(
    instructions: &[Input],
    v1: usize,
    v2: usize,
) -> (Option<usize>, HashMap<usize, usize>) {
    let mut bots: HashMap<usize, Input> = HashMap::new();
    let mut microchips: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut outputs: HashMap<usize, usize> = HashMap::new();

    for instruction in instructions {
        match instruction {
            Input::Bin(value, Output::Bot(nr)) => {
                microchips.entry(*nr).or_insert_with(Vec::new).push(*value);
            }
            Input::Bin(value, Output::Bin(nr)) => {
                outputs.insert(*nr, *value);
            }
            Input::Bot(nr, _, _) => {
                bots.insert(*nr, instruction.clone());
                microchips.entry(*nr).or_insert_with(Vec::new);
            }
        }
    }

    let mut should_continue = true;
    let mut requested_bot: Option<usize> = None;
    while should_continue {
        let mut microchips_to_update: HashMap<usize, Vec<usize>> = HashMap::new();
        should_continue = false;
        for (bot, chips) in microchips.iter_mut().filter(|(_, chips)| chips.len() == 2) {
            should_continue = true;
            let instruction = bots.get(bot).unwrap();
            let chip1 = chips.pop().unwrap();
            let chip2 = chips.pop().unwrap();

            if requested_bot.is_none()
                && ((chip1 == v1 && chip2 == v2) || (chip1 == v2 && chip2 == v1))
            {
                requested_bot = Some(*bot);
            }

            match instruction {
                Input::Bin(_, _) => unreachable!(),
                Input::Bot(_, lower, higher) => {
                    match lower {
                        Output::Bot(nr) => {
                            microchips_to_update
                                .entry(*nr)
                                .or_insert_with(Vec::new)
                                .push(chip1.min(chip2));
                        }
                        Output::Bin(nr) => {
                            outputs.insert(*nr, chip1.min(chip2));
                        }
                    }

                    match higher {
                        Output::Bot(nr) => {
                            microchips_to_update
                                .entry(*nr)
                                .or_insert_with(Vec::new)
                                .push(chip1.max(chip2));
                        }
                        Output::Bin(nr) => {
                            outputs.insert(*nr, chip1.max(chip2));
                        }
                    }
                }
            }
        }
        for (k, v) in microchips_to_update {
            microchips.entry(k).or_insert_with(Vec::new).extend(v);
        }
    }

    (requested_bot, outputs)
}

#[cfg(test)]
mod tests {
    use rdcl_aoc_helpers::input::WithAsRecords;

    use super::*;

    #[test]
    fn test_evaluate() {
        let instructions = vec![
            "value 5 goes to bot 2",
            "bot 2 gives low to bot 1 and high to bot 0",
            "value 3 goes to bot 1",
            "bot 1 gives low to output 1 and high to bot 0",
            "bot 0 gives low to output 2 and high to output 0",
            "value 2 goes to bot 2",
        ]
        .as_records::<Input>()
        .unwrap();

        let (bot, outputs) = evaluate(&instructions, 5, 2);
        assert_eq!(bot, Some(2));
        let mut expected: HashMap<usize, usize> = HashMap::new();
        expected.insert(0, 5);
        expected.insert(1, 2);
        expected.insert(2, 3);
        assert_eq!(outputs, expected);
    }
}
