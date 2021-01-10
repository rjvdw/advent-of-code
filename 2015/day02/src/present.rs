use std::str::FromStr;

use rdcl_aoc_helpers::err_parse_error;
use rdcl_aoc_helpers::error::ParseError;

pub struct Present(u32, u32, u32);

impl Present {
    pub fn get_required_materials(&self) -> (u32, u32) {
        (
            self.get_required_wrapping_paper(),
            self.get_required_ribbon(),
        )
    }

    pub fn get_required_wrapping_paper(&self) -> u32 {
        self.get_area() + self.get_smallest_side_area()
    }

    pub fn get_required_ribbon(&self) -> u32 {
        self.get_shortest_perimeter() + self.get_bow_length()
    }

    fn get_area(&self) -> u32 {
        2 * (self.0 * self.1 + self.1 * self.2 + self.2 * self.0)
    }

    fn get_smallest_side_area(&self) -> u32 {
        let sides = [self.0 * self.1, self.1 * self.2, self.2 * self.0];
        sides.iter().min().cloned().unwrap()
    }

    fn get_shortest_perimeter(&self) -> u32 {
        2 * (self.0 + self.1 + self.2 - [self.0, self.1, self.2].iter().max().cloned().unwrap())
    }

    fn get_bow_length(&self) -> u32 {
        self.0 * self.1 * self.2
    }
}

impl FromStr for Present {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut values = Vec::new();
        for dim in s.split('x') {
            values.push(dim.parse::<u32>()?);
        }
        if values.len() != 3 {
            return err_parse_error!("Present with dimensions {} could not be parsed", s);
        }
        Ok(Present(values[0], values[1], values[2]))
    }
}
