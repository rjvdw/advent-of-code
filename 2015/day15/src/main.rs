use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;
use rdcl_aoc_helpers::input::WithReadLines;

use crate::ingredient::Ingredient;

mod ingredient;

fn main() {
    let args = get_args(&["<input file>", "<nr teaspoons>", "<calories>"], 1);
    let ingredients = File::open(&args[1])
        .read_lines(1)
        .collect::<Vec<Ingredient>>();
    let nr_teaspoons = args[2].parse::<usize>().or_exit_with(1);
    let calories = args[3].parse::<i32>().or_exit_with(1);

    let (recipe, score) = optimize_recipe(&ingredients, nr_teaspoons);
    println!("The optimal recipe is: {} (score: {}).", recipe, score);

    match optimize_recipe_with_calorie_restriction(&ingredients, nr_teaspoons, calories) {
        Some((recipe, score)) => println!(
            "The optimal recipe using {} calories is: {} (score: {}).",
            calories, recipe, score
        ),
        None => eprintln!("There is no valid recipe using only {} calories.", calories),
    }
}

fn optimize_recipe(ingredients: &[Ingredient], nr_teaspoons: usize) -> (String, i32) {
    try_ingredient(Ingredient::default(), ingredients, nr_teaspoons, None).unwrap()
}

fn optimize_recipe_with_calorie_restriction(
    ingredients: &[Ingredient],
    nr_teaspoons: usize,
    calories: i32,
) -> Option<(String, i32)> {
    try_ingredient(
        Ingredient::default(),
        ingredients,
        nr_teaspoons,
        Some(calories),
    )
}

fn try_ingredient(
    recipe: Ingredient,
    ingredients: &[Ingredient],
    nr_teaspoons: usize,
    calorie_restriction: Option<i32>,
) -> Option<(String, i32)> {
    let (ingredient, rest) = ingredients.split_first().unwrap();

    if rest.is_empty() {
        Some(recipe + ingredient.clone() * (nr_teaspoons as i32))
            .filter(|recipe| {
                calorie_restriction.is_none()
                    || recipe.get_calories() == calorie_restriction.unwrap()
            })
            .map(|recipe| (recipe.get_name(), recipe.p1_score()))
    } else {
        let mut best_so_far: (String, i32) = (Default::default(), Default::default());

        for i in 0..=nr_teaspoons {
            if let Some(result) = try_ingredient(
                recipe.clone() + ingredient.clone() * (i as i32),
                rest,
                nr_teaspoons - i,
                calorie_restriction,
            ) {
                if result.1 > best_so_far.1 {
                    best_so_far = result;
                }
            }
        }

        if best_so_far.0.is_empty() {
            None
        } else {
            Some(best_so_far)
        }
    }
}

#[cfg(test)]
mod tests {
    use rdcl_aoc_helpers::input::WithAsRecords;

    use super::*;

    #[test]
    fn test_without_calorie_restriction() {
        let ingredients = vec![
            "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8",
            "Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3",
        ]
        .as_records::<Ingredient>()
        .unwrap();

        let (recipe, score) = optimize_recipe(&ingredients, 100);

        assert_eq!(
            (recipe, score),
            ("44x Butterscotch, 56x Cinnamon".to_string(), 62842880)
        );
    }

    #[test]
    fn test_with_calorie_restriction() {
        let ingredients = vec![
            "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8",
            "Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3",
        ]
        .as_records::<Ingredient>()
        .unwrap();

        let (recipe, score) =
            optimize_recipe_with_calorie_restriction(&ingredients, 100, 500).unwrap();

        assert_eq!(
            (recipe, score),
            ("40x Butterscotch, 60x Cinnamon".to_string(), 57600000)
        );
    }
}
