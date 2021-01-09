use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::input::WithReadLines;

use crate::program::Program;

mod program;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let programs = File::open(&args[1]).read_lines(1).collect::<Vec<Program>>();

    match find_bottom_program(&programs) {
        Some(program) => println!("The program at the bottom has name {}.", program.name),
        None => eprintln!("There is no program at the bottom."),
    }

    match find_program_with_wrong_weight(&programs) {
        Some((program, weight, should_be)) => println!(
            "Program {} has weight {}, while it should have weight {}.",
            program.name, weight, should_be
        ),
        None => eprintln!("Could not find any program with the wrong weight."),
    }
}

fn find_bottom_program(programs: &[Program]) -> Option<Program> {
    programs
        .iter()
        .find(|p1| !programs.iter().any(|p2| p2.children.contains(&p1.name)))
        .cloned()
}

fn find_program_with_wrong_weight(programs: &[Program]) -> Option<(Program, usize, usize)> {
    let mut programs_map = HashMap::new();
    let mut weights = HashMap::new();

    for program in programs {
        programs_map.insert(program.name.to_string(), program.clone());
    }

    let root = find_bottom_program(programs)?;
    let mut in_between = InBetweenResult {
        program: root.clone(),
        total_weight: root.get_weight(&programs_map, &mut weights),
        correct_weight: 0,
    };

    let mut programs_per_weight: HashMap<usize, Vec<String>> = HashMap::with_capacity(2);

    while !in_between.program.children.is_empty() {
        programs_per_weight.clear();
        for name in &in_between.program.children {
            let child = programs_map.get(name).unwrap();
            let weight = child.get_weight(&programs_map, &mut weights);
            programs_per_weight
                .entry(weight)
                .or_insert_with(Vec::new)
                .push(name.to_string());
        }

        match programs_per_weight.len() {
            1 => {
                return Some(in_between.get_final_result());
            }
            2 => {
                let min = programs_per_weight
                    .iter()
                    .min_by_key(|(weight, _)| *weight)?;
                let max = programs_per_weight
                    .iter()
                    .max_by_key(|(weight, _)| *weight)?;

                let (total_weight, correct_weight, children) = match in_between.compare() {
                    Ordering::Less => (*min.0, *max.0, min.1),
                    Ordering::Greater => (*max.0, *min.0, max.1),
                    Ordering::Equal => unreachable!(),
                };

                assert_eq!(children.len(), 1);
                let child = children.first()?;

                in_between = InBetweenResult {
                    program: programs_map.get(child)?.clone(),
                    total_weight,
                    correct_weight,
                };
            }
            _ => panic!("Invalid distribution of weights: {:?}", programs_per_weight),
        }
    }

    None
}

struct InBetweenResult {
    program: Program,
    total_weight: usize,
    correct_weight: usize,
}

impl InBetweenResult {
    fn compare(&self) -> Ordering {
        self.total_weight.cmp(&self.correct_weight)
    }

    fn get_should_be(&self) -> usize {
        if self.total_weight > self.correct_weight {
            self.program.weight + self.correct_weight - self.total_weight
        } else {
            self.program.weight - (self.total_weight - self.correct_weight)
        }
    }

    fn get_final_result(&self) -> (Program, usize, usize) {
        (
            self.program.clone(),
            self.total_weight,
            self.get_should_be(),
        )
    }
}

#[cfg(test)]
mod tests {
    use rdcl_aoc_helpers::input::WithAsRecords;

    use super::*;

    #[test]
    fn test_find_bottom_program() {
        let programs = vec![
            "pbga (66)",
            "xhth (57)",
            "ebii (61)",
            "havc (66)",
            "ktlj (57)",
            "fwft (72) -> ktlj, cntj, xhth",
            "qoyq (66)",
            "padx (45) -> pbga, havc, qoyq",
            "tknk (41) -> ugml, padx, fwft",
            "jptl (61)",
            "ugml (68) -> gyxo, ebii, jptl",
            "gyxo (61)",
            "cntj (57)",
        ]
        .as_records::<Program>()
        .unwrap();

        assert_eq!(find_bottom_program(&programs).unwrap().name, "tknk");
    }

    #[test]
    fn test_find_program_with_wrong_weight() {
        let programs = vec![
            "pbga (66)",
            "xhth (57)",
            "ebii (61)",
            "havc (66)",
            "ktlj (57)",
            "fwft (72) -> ktlj, cntj, xhth",
            "qoyq (66)",
            "padx (45) -> pbga, havc, qoyq",
            "tknk (41) -> ugml, padx, fwft",
            "jptl (61)",
            "ugml (68) -> gyxo, ebii, jptl",
            "gyxo (61)",
            "cntj (57)",
        ]
        .as_records::<Program>()
        .unwrap();

        let (program, weight, should_be) = find_program_with_wrong_weight(&programs).unwrap();
        assert_eq!(program.name, "ugml");
        assert_eq!(weight, 251);
        assert_eq!(should_be, 60);
    }
}
