use std::collections::HashSet;
use std::str::FromStr;

use rdcl_aoc_helpers::parse_error::ParseError;

#[derive(Debug, Clone)]
pub struct Food {
    pub ingredients: HashSet<String>,
    pub allergens: HashSet<String>,
}

impl Food {
    pub fn contains_ingredient(&self, ingredient: String) -> bool {
        self.ingredients.contains(&ingredient)
    }
}

impl FromStr for Food {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.find("(contains") {
            Some(idx) => {
                let ingredients = s[..idx].trim().split(' ').map(|v| v.to_string()).collect();

                let allergens = s[idx + 9..s.len() - 1]
                    .trim()
                    .split(", ")
                    .map(|v| v.to_string())
                    .collect();

                Ok(Food {
                    ingredients,
                    allergens,
                })
            }
            None => Err(ParseError(format!("Invalid input: {}", s))),
        }
    }
}
