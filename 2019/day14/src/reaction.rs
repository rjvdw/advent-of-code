use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use rdcl_aoc_helpers::error::ParseError;

#[derive(Debug, Clone)]
pub struct Reaction {
    pub input: Vec<(usize, String)>,
    pub output_chemical: String,
    pub output_quantity: usize,
}

impl Display for Reaction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} => {} {}",
            self.input.iter().map(|(q, c)| format!("{} {}", q, c)).fold(
                String::new(),
                |mut acc, p| {
                    if !acc.is_empty() {
                        acc.push_str(", ");
                    }
                    acc.push_str(&p);
                    acc
                },
            ),
            self.output_quantity,
            self.output_chemical
        )
    }
}

impl FromStr for Reaction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut reaction = Reaction {
            input: vec![],
            output_chemical: String::new(),
            output_quantity: 0,
        };
        let mut parsing = Parsing::InputQuantity;
        let mut input_quantity = 0;
        let mut collecting = String::new();

        for ch in s.chars() {
            match parsing {
                Parsing::InputQuantity => {
                    if ch == ' ' {
                        input_quantity = collecting.parse()?;
                        collecting.clear();
                        parsing = Parsing::InputChemical;
                    } else if ch == '=' {
                        parsing = Parsing::Arrow;
                    } else {
                        collecting.push(ch);
                    }
                }
                Parsing::InputChemical => {
                    if ch == ',' {
                        // noop
                    } else if ch == ' ' {
                        reaction
                            .input
                            .push((input_quantity, collecting.to_string()));
                        collecting.clear();
                        parsing = Parsing::InputQuantity;
                    } else {
                        collecting.push(ch);
                    }
                }
                Parsing::Arrow => {
                    if ch == ' ' {
                        parsing = Parsing::OutputQuantity
                    }
                }
                Parsing::OutputQuantity => {
                    if ch == ' ' {
                        reaction.output_quantity = collecting.parse()?;
                        parsing = Parsing::OutputChemical;
                    } else {
                        collecting.push(ch);
                    }
                }
                Parsing::OutputChemical => reaction.output_chemical.push(ch),
            }
        }

        Ok(reaction)
    }
}

enum Parsing {
    InputQuantity,
    InputChemical,
    Arrow,
    OutputQuantity,
    OutputChemical,
}
