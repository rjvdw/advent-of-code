use std::ops::{Add, Mul};
use std::str::FromStr;

use rdcl_aoc_helpers::error::ParseError;

#[derive(Debug, Clone)]
pub struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl Ingredient {
    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    pub fn p1_score(&self) -> i32 {
        self.capacity.max(0) * self.durability.max(0) * self.flavor.max(0) * self.texture.max(0)
    }

    pub fn get_calories(&self) -> i32 {
        self.calories
    }
}

impl Default for Ingredient {
    fn default() -> Self {
        Ingredient {
            name: Default::default(),
            capacity: Default::default(),
            durability: Default::default(),
            flavor: Default::default(),
            texture: Default::default(),
            calories: Default::default(),
        }
    }
}

impl Mul<i32> for Ingredient {
    type Output = Ingredient;

    fn mul(self, rhs: i32) -> Self::Output {
        Ingredient {
            name: format!("{}x {}", rhs, self.name),
            capacity: self.capacity * rhs,
            durability: self.durability * rhs,
            flavor: self.flavor * rhs,
            texture: self.texture * rhs,
            calories: self.calories * rhs,
        }
    }
}

impl Add<Ingredient> for Ingredient {
    type Output = Ingredient;

    fn add(self, rhs: Ingredient) -> Self::Output {
        Ingredient {
            name: if self.name.is_empty() {
                rhs.name
            } else {
                format!("{}, {}", self.name, rhs.name)
            },
            capacity: self.capacity + rhs.capacity,
            durability: self.durability + rhs.durability,
            flavor: self.flavor + rhs.flavor,
            texture: self.texture + rhs.texture,
            calories: self.calories + rhs.calories,
        }
    }
}

impl FromStr for Ingredient {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(idx) = s.find(':') {
            let mut ingredient = Ingredient {
                name: s[..idx].to_string(),
                ..Default::default()
            };
            for property in s[idx + 2..].split(", ") {
                if let Some(idx) = property.find(' ') {
                    let value = property[idx + 1..].parse::<i32>()?;
                    match &property[..idx] {
                        "capacity" => ingredient.capacity = value,
                        "durability" => ingredient.durability = value,
                        "flavor" => ingredient.flavor = value,
                        "texture" => ingredient.texture = value,
                        "calories" => ingredient.calories = value,
                        _ => {
                            return Err(ParseError(format!(
                                "Invalid property '{}' in ingredient: {}",
                                property, s
                            )));
                        }
                    };
                } else {
                    return Err(ParseError(format!("Invalid ingredient: {}", s)));
                }
            }
            Ok(ingredient)
        } else {
            Err(ParseError(format!("Invalid ingredient: {}", s)))
        }
    }
}
