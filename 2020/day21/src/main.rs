extern crate rdcl_aoc_helpers;

use std::collections::{HashMap, HashSet};
use std::env;
use std::process::exit;

use rdcl_aoc_helpers::handle_result;
use rdcl_aoc_helpers::read::read_input;

use crate::food::Food;

mod food;

/// https://adventofcode.com/2020/day/21
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <input file>", &args[0]);
        exit(1);
    }

    let foods: Vec<Food> = handle_result(read_input(&args[1]));
    let possible_ingredients_per_allergen = find_possible_ingredients_per_allergen(&foods);
    let safe_ingredients = find_safe_ingredients(&foods, &possible_ingredients_per_allergen);

    // println!("The safe ingredients are: {:?}", safe_ingredients);
    println!(
        "The safe ingredients occur this many times: {}",
        count_occurences(&foods, &safe_ingredients)
    );

    let (dangerous_ingredients, is_complete) =
        find_dangerous_ingredients(&possible_ingredients_per_allergen);

    println!(
        "The {} canonical dangerous ingredients list is: '{}'",
        if is_complete {
            "complete"
        } else {
            "incomplete"
        },
        dangerous_ingredients.join(",")
    );
}

fn find_possible_ingredients_per_allergen(foods: &[Food]) -> HashMap<String, HashSet<String>> {
    let mut possible_ingredients_per_allergen: HashMap<String, HashSet<String>> = HashMap::new();

    for food in foods {
        for allergen in &food.allergens {
            match possible_ingredients_per_allergen.get(allergen) {
                Some(candidates) => {
                    let mut new_candidates = HashSet::new();
                    for candidate in candidates {
                        if food.contains_ingredient(candidate.to_string()) {
                            new_candidates.insert(candidate.to_string());
                        }
                    }
                    possible_ingredients_per_allergen.insert(allergen.to_string(), new_candidates);
                }
                None => {
                    possible_ingredients_per_allergen.insert(
                        allergen.to_string(),
                        food.ingredients.iter().map(|v| v.to_string()).collect(),
                    );
                }
            }
        }
    }

    possible_ingredients_per_allergen
}

fn find_safe_ingredients(
    foods: &[Food],
    possible_ingredients_per_allergen: &HashMap<String, HashSet<String>>,
) -> HashSet<String> {
    foods
        .iter()
        .flat_map(|f| f.ingredients.iter().map(|i| i.to_string()))
        .filter(|i| {
            possible_ingredients_per_allergen
                .iter()
                .all(|(_, ingredients)| !ingredients.contains(i))
        })
        .collect()
}

fn count_occurences(foods: &[Food], ingredients: &HashSet<String>) -> usize {
    let mut count = 0;

    for food in foods {
        for ingredient in ingredients {
            if food.contains_ingredient(ingredient.to_string()) {
                count += 1;
            }
        }
    }

    count
}

fn find_dangerous_ingredients(
    possible_ingredients_per_allergen: &HashMap<String, HashSet<String>>,
) -> (Vec<String>, bool) {
    let mut processed_allergens: HashSet<String> = HashSet::new();
    let mut dangerous_ingredients: HashSet<String> = HashSet::new();

    let mut mapping: Vec<(String, String)> = Vec::new();

    let mut done = false;
    while !done {
        done = true;
        for (allergen, ingredients) in possible_ingredients_per_allergen {
            if !processed_allergens.contains(allergen) {
                let ingredients: Vec<String> = ingredients
                    .iter()
                    .filter(|&i| !dangerous_ingredients.contains(i))
                    .map(|v| v.to_string())
                    .collect();

                if ingredients.len() == 1 {
                    processed_allergens.insert(allergen.to_string());
                    dangerous_ingredients.insert(ingredients[0].to_string());

                    mapping.push((allergen.to_string(), ingredients[0].to_string()));

                    done = false;
                }
            }
        }
    }

    mapping.sort_unstable_by_key(|(allergen, _)| allergen.to_string());

    (
        mapping
            .iter()
            .map(|(_, ingredient)| ingredient)
            .map(|v| v.to_string())
            .collect(),
        dangerous_ingredients.len() == possible_ingredients_per_allergen.len(),
    )
}

#[cfg(test)]
mod tests {
    use rdcl_aoc_helpers::parse::parse_input;

    use super::*;

    #[test]
    fn test_find_possible_ingredients_per_allergen() {
        let foods = get_input();
        let allergens = find_possible_ingredients_per_allergen(&foods);

        let mut expected: HashMap<String, HashSet<String>> = HashMap::new();
        expected
            .entry("dairy".to_string())
            .or_insert_with(HashSet::new)
            .insert("mxmxvkd".to_string());
        expected
            .entry("soy".to_string())
            .or_insert_with(HashSet::new)
            .insert("sqjhc".to_string());
        expected
            .entry("soy".to_string())
            .or_insert_with(HashSet::new)
            .insert("fvjkl".to_string());
        expected
            .entry("fish".to_string())
            .or_insert_with(HashSet::new)
            .insert("sqjhc".to_string());
        expected
            .entry("fish".to_string())
            .or_insert_with(HashSet::new)
            .insert("mxmxvkd".to_string());

        assert_eq!(allergens, expected);
    }

    #[test]
    fn test_find_safe_ingredients() {
        let foods = get_input();
        let allergens = find_possible_ingredients_per_allergen(&foods);

        let mut expected: HashSet<String> = HashSet::new();
        expected.insert("kfcds".to_string());
        expected.insert("nhms".to_string());
        expected.insert("sbzzf".to_string());
        expected.insert("trh".to_string());

        assert_eq!(find_safe_ingredients(&foods, &allergens), expected);
    }

    #[test]
    fn test_find_dangerous_ingredients() {
        let foods = get_input();
        let allergens = find_possible_ingredients_per_allergen(&foods);

        assert_eq!(
            find_dangerous_ingredients(&allergens),
            (
                vec![
                    "mxmxvkd".to_string(),
                    "sqjhc".to_string(),
                    "fvjkl".to_string()
                ],
                true
            )
        );
    }

    fn get_input() -> Vec<Food> {
        parse_input(vec![
            "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)",
            "trh fvjkl sbzzf mxmxvkd (contains dairy)",
            "sqjhc fvjkl (contains soy)",
            "sqjhc mxmxvkd sbzzf (contains fish)",
        ])
        .unwrap()
    }
}