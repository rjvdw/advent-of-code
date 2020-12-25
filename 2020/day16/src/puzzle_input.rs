use std::collections::HashMap;
use std::fmt;

use rdcl_aoc_helpers::from_multiline_str::FromMultilineStr;
use rdcl_aoc_helpers::parse_error::ParseError;

#[derive(Debug)]
enum ParseState {
    FieldValues,
    YourTicket,
    NearbyTickets,
}

pub struct PuzzleInput {
    pub possible_field_values: HashMap<String, Vec<(u32, u32)>>,
    pub your_ticket: Vec<u32>,
    pub nearby_tickets: Vec<Vec<u32>>,
    parse_state: ParseState,
}

impl fmt::Debug for PuzzleInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut keys = self.possible_field_values.keys().collect::<Vec<&String>>();
        keys.sort_unstable();

        for key in keys {
            write!(f, "{}: ", key)?;
            let values = self
                .possible_field_values
                .get(key)
                .unwrap()
                .iter()
                .map(|&(lower, upper)| format!("{}-{}", lower, upper));

            let mut begin = true;
            for value in values {
                if !begin {
                    write!(f, " or ")?;
                }
                begin = false;
                write!(f, "{}", value)?;
            }
            writeln!(f)?;
        }

        write!(
            f,
            "\nyour ticket:\n{:?}\n\nnearby tickets:",
            self.your_ticket,
        )?;

        for ticket in self.nearby_tickets.to_vec() {
            write!(f, "\n{:?}", ticket)?;
        }

        Ok(())
    }
}

impl PuzzleInput {
    /// Filters out nearby tickets that are invalid, and returns the sum of the invalid values.
    pub fn filter_out_invalid_nearby_tickets(&mut self) -> u32 {
        let mut filtered = Vec::new();
        let mut sum = 0;

        for ticket in self.nearby_tickets.to_vec() {
            let (valid, count) = self.count_invalid_values_in_ticket(&ticket);
            if valid {
                filtered.push(ticket);
            } else {
                sum += count;
            }
        }

        self.nearby_tickets = filtered;
        sum
    }

    /// Checks whether all values at a given position match the restrictions on the field `key`.
    pub fn matches_all_tickets(&self, key: &str, pos: usize) -> bool {
        match self.possible_field_values.get(key) {
            Some(ranges) => {
                falls_within_ranges(ranges, self.your_ticket[pos])
                    && self
                        .nearby_tickets
                        .iter()
                        .all(|ticket| falls_within_ranges(ranges, ticket[pos]))
            }
            None => false,
        }
    }

    fn count_invalid_values_in_ticket(&self, ticket: &[u32]) -> (bool, u32) {
        let mut valid = true;
        let mut count = 0;

        for &value in ticket {
            let ranges = self
                .possible_field_values
                .values()
                .flatten()
                .copied()
                .collect::<Vec<(u32, u32)>>();

            if !falls_within_ranges(&ranges, value) {
                valid = false;
                count += value;
            }
        }

        (valid, count)
    }
}

impl FromMultilineStr for PuzzleInput {
    const DISCARD_FIRST_RECORD: bool = false;

    type Err = ParseError;

    fn new() -> Self {
        PuzzleInput {
            possible_field_values: HashMap::new(),
            your_ticket: Vec::new(),
            nearby_tickets: Vec::new(),
            parse_state: ParseState::FieldValues,
        }
    }

    fn indicates_new_record(_line: &str) -> bool {
        false
    }

    fn parse(&mut self, line: &str) -> Result<(), Self::Err> {
        if line.is_empty() {
            return Ok(());
        }

        match self.parse_state {
            ParseState::FieldValues => {
                if line == "your ticket:" {
                    self.parse_state = ParseState::YourTicket;
                    Ok(())
                } else {
                    match line.find(": ") {
                        Some(idx) => {
                            let key = line[..idx].to_string();
                            let ranges = line[idx + 2..].split(" or ");
                            let mut value: Vec<(u32, u32)> = Vec::new();

                            for range in ranges {
                                match range.find('-') {
                                    Some(idx) => {
                                        let start = range[..idx].parse::<u32>()?;
                                        let end = range[idx + 1..].parse::<u32>()?;

                                        value.push((start, end));
                                    }
                                    None => {
                                        return Err(ParseError(format!(
                                            "Invalid input line: {}",
                                            line
                                        )));
                                    }
                                }
                            }

                            self.possible_field_values.insert(key, value);

                            Ok(())
                        }
                        None => Err(ParseError(format!("Invalid input line: {}", line))),
                    }
                }
            }
            ParseState::YourTicket => {
                if line == "nearby tickets:" {
                    self.parse_state = ParseState::NearbyTickets;
                    Ok(())
                } else {
                    for number in line.split(',') {
                        self.your_ticket.push(number.parse::<u32>()?);
                    }

                    Ok(())
                }
            }
            ParseState::NearbyTickets => {
                let mut ticket = Vec::new();
                for number in line.split(',') {
                    ticket.push(number.parse::<u32>()?);
                }

                self.nearby_tickets.push(ticket);

                Ok(())
            }
        }
    }
}

fn falls_within_ranges(ranges: &[(u32, u32)], value: u32) -> bool {
    for &(lower, upper) in ranges {
        if value >= lower && value <= upper {
            return true;
        }
    }

    false
}
