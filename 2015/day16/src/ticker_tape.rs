use std::collections::HashMap;

use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::input::MultilineFromStr;

#[derive(Debug)]
pub struct TickerTape(HashMap<String, usize>);

impl TickerTape {
    pub fn as_map(&self) -> HashMap<String, usize> {
        self.0.clone()
    }
}

impl MultilineFromStr for TickerTape {
    type Err = ParseError;

    fn new() -> Self {
        TickerTape(HashMap::new())
    }

    fn indicates_new_record(&self, _line: &str) -> bool {
        false
    }

    fn parse(&mut self, line: &str) -> Result<(), Self::Err> {
        if let Some(idx) = line.find(':') {
            let property = line[..idx].to_string();
            let value = line[idx + 2..].parse::<usize>()?;
            self.0.insert(property, value);
            Ok(())
        } else {
            Err(ParseError(format!(
                "Invalid input line in ticker tape: {}",
                line
            )))
        }
    }
}
