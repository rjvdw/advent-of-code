use std::collections::HashMap;
use std::str::FromStr;

use rdcl_aoc_helpers::error::ParseError;

#[derive(Debug)]
pub struct Sue {
    name: String,
    properties: HashMap<String, usize>,
}

impl Sue {
    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    pub fn matches_p1(&self, ticker_tape: &HashMap<String, usize>) -> bool {
        for (key, value) in &self.properties {
            if ticker_tape.get(key).cloned().unwrap() != *value {
                return false;
            }
        }

        true
    }

    pub fn matches_p2(&self, ticker_tape: &HashMap<String, usize>) -> bool {
        for (key, value) in &self.properties {
            let value_left = ticker_tape.get(key).cloned().unwrap();
            let value_right = *value;

            if key.eq("cats") || key.eq("trees") {
                if value_left >= value_right {
                    return false;
                }
            } else if key.eq("pomeranians") || key.eq("goldfish") {
                if value_left <= value_right {
                    return false;
                }
            } else if value_left != *value {
                return false;
            }
        }

        true
    }
}

impl FromStr for Sue {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(idx) = s.find(':') {
            let mut sue = Sue {
                name: s[..idx].to_string(),
                properties: HashMap::new(),
            };

            for key_value_pair in s[idx + 2..].split(", ") {
                if let Some(idx) = key_value_pair.find(':') {
                    let property = key_value_pair[..idx].to_string();
                    let value = key_value_pair[idx + 2..].parse::<usize>()?;
                    sue.properties.insert(property, value);
                } else {
                    return Err(ParseError(format!(
                        "Invalid key value pair '{}' Sue: {}",
                        key_value_pair, s
                    )));
                }
            }

            Ok(sue)
        } else {
            Err(ParseError(format!("Invalid Sue: {}", s)))
        }
    }
}
