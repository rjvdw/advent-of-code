extern crate helpers;

use std::collections::{HashMap, HashSet};
use std::env;
use std::process::exit;

use helpers::{handle_result, read_input};

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

    println!(
        "The canonical dangerous ingredients list is: '{}'",
        find_dangerous_ingredients(&possible_ingredients_per_allergen).join(",")
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
) -> Vec<String> {
    let mut processed_allergens: HashSet<String> = HashSet::new();
    let mut dangerous_ingredients: HashSet<String> = HashSet::new();

    let mut mapping: Vec<(String, String)> = Vec::new();

    // assumption: there is a solution (otherwise this will loop indefinitely)
    while dangerous_ingredients.len() != possible_ingredients_per_allergen.len() {
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
                }
            }
        }
    }

    mapping.sort_unstable_by_key(|(allergen, _)| allergen.to_string());

    mapping
        .iter()
        .map(|(_, ingredient)| ingredient)
        .map(|v| v.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use helpers::parse_input;

    use super::*;

    #[test]
    fn test_find_possible_ingredients_per_allergen() {
        let foods = get_input();
        let allergens = find_possible_ingredients_per_allergen(&foods);

        let mut expected: HashMap<String, HashSet<String>> = HashMap::new();
        expected
            .entry("dairy".to_string())
            .or_insert(HashSet::new())
            .insert("mxmxvkd".to_string());
        expected
            .entry("soy".to_string())
            .or_insert(HashSet::new())
            .insert("sqjhc".to_string());
        expected
            .entry("soy".to_string())
            .or_insert(HashSet::new())
            .insert("fvjkl".to_string());
        expected
            .entry("fish".to_string())
            .or_insert(HashSet::new())
            .insert("sqjhc".to_string());
        expected
            .entry("fish".to_string())
            .or_insert(HashSet::new())
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
            vec!["mxmxvkd", "sqjhc", "fvjkl"]
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
