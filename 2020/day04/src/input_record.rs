use std::fmt;

use rdcl_aoc_helpers::err_parse_error;
use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::input::MultilineFromStr;

use crate::validators::{
    valid_color, valid_eye_color, valid_height, valid_passport_id, valid_year,
};

struct PassportField {
    value: Option<String>,
    validator: fn(&String) -> bool,
}

impl PassportField {
    fn none(validator: fn(&String) -> bool) -> Self {
        PassportField {
            value: None,
            validator,
        }
    }

    fn set_value(&mut self, value: String) {
        self.value = Some(value);
    }

    fn is_present(&self) -> bool {
        self.value.is_some()
    }

    fn is_valid(&self) -> bool {
        !self.is_present() || (self.validator)(self.value.as_ref().unwrap())
    }

    fn is_present_and_valid(&self) -> bool {
        self.is_present() && (self.validator)(self.value.as_ref().unwrap())
    }
}

impl fmt::Display for PassportField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.value)
    }
}

impl fmt::Debug for PassportField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.value)
    }
}

#[derive(Debug)]
pub struct Passport {
    byr: PassportField,
    iyr: PassportField,
    eyr: PassportField,
    hgt: PassportField,
    hcl: PassportField,
    ecl: PassportField,
    pid: PassportField,
    cid: PassportField,
}

impl Passport {
    pub fn has_required_fields(&self) -> bool {
        self.byr.is_present()
            && self.iyr.is_present()
            && self.eyr.is_present()
            && self.hgt.is_present()
            && self.hcl.is_present()
            && self.ecl.is_present()
            && self.pid.is_present()
    }

    pub fn is_valid(&self) -> bool {
        self.byr.is_present_and_valid()
            && self.iyr.is_present_and_valid()
            && self.eyr.is_present_and_valid()
            && self.hgt.is_present_and_valid()
            && self.hcl.is_present_and_valid()
            && self.ecl.is_present_and_valid()
            && self.pid.is_present_and_valid()
            && self.cid.is_valid()
    }
}

impl MultilineFromStr for Passport {
    type Err = ParseError;

    fn new() -> Self {
        Passport {
            byr: PassportField::none(|v| valid_year(v, 1920, 2002)),
            iyr: PassportField::none(|v| valid_year(v, 2010, 2020)),
            eyr: PassportField::none(|v| valid_year(v, 2020, 2030)),
            hgt: PassportField::none(|v| valid_height(v, 59, 76, 150, 193)),
            hcl: PassportField::none(|v| valid_color(v)),
            ecl: PassportField::none(|v| valid_eye_color(v)),
            pid: PassportField::none(|v| valid_passport_id(v)),
            cid: PassportField::none(|_| true),
        }
    }

    fn indicates_new_record(&self, line: &str) -> bool {
        line.is_empty()
    }

    fn parse(&mut self, line: &str) -> Result<(), Self::Err> {
        if !line.is_empty() {
            for tuple in line.split(' ') {
                match tuple.find(':') {
                    Some(idx) => {
                        let key = &tuple[..idx];
                        let value = tuple[idx + 1..].to_string();

                        #[allow(clippy::unit_arg)]
                        match key {
                            "byr" => Ok(self.byr.set_value(value)),
                            "iyr" => Ok(self.iyr.set_value(value)),
                            "eyr" => Ok(self.eyr.set_value(value)),
                            "hgt" => Ok(self.hgt.set_value(value)),
                            "hcl" => Ok(self.hcl.set_value(value)),
                            "ecl" => Ok(self.ecl.set_value(value)),
                            "pid" => Ok(self.pid.set_value(value)),
                            "cid" => Ok(self.cid.set_value(value)),
                            _ => err_parse_error!("Invalid key '{}' in line '{}'", key, line),
                        }?
                    }
                    None => {
                        return err_parse_error!("Invalid tuple '{}' in line '{}'", tuple, line);
                    }
                }
            }
        }

        Ok(())
    }
}
