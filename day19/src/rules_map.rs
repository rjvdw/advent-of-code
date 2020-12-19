use std::collections::HashMap;
use std::process::exit;

use crate::rule::Rule;

pub trait RulesMap {
    fn get_rule(&self, key: &usize) -> &Rule;
}

impl RulesMap for HashMap<usize, Rule> {
    fn get_rule(&self, key: &usize) -> &Rule {
        match self.get(key) {
            Some(rule) => rule,
            None => {
                eprintln!("Non existent rule encountered: {}", key);
                exit(1);
            }
        }
    }
}
