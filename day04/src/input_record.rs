use std::fmt;

use helpers::FromMultilineStr;

use crate::validators::{
    valid_color, valid_eye_color, valid_height, valid_passport_id, valid_year,
};

struct InputRecordField {
    value: Option<String>,
    validator: fn(&String) -> bool,
}

impl InputRecordField {
    fn none(validator: fn(&String) -> bool) -> Self {
        InputRecordField {
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

impl fmt::Display for InputRecordField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.value)
    }
}

impl fmt::Debug for InputRecordField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.value)
    }
}

#[derive(Debug)]
pub struct InputRecord {
    byr: InputRecordField,
    iyr: InputRecordField,
    eyr: InputRecordField,
    hgt: InputRecordField,
    hcl: InputRecordField,
    ecl: InputRecordField,
    pid: InputRecordField,
    cid: InputRecordField,
}

impl InputRecord {
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

#[derive(Debug)]
pub struct InputRecordError {
    msg: String,
}

impl fmt::Display for InputRecordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl FromMultilineStr for InputRecord {
    type Err = InputRecordError;

    fn new() -> Self {
        InputRecord {
            byr: InputRecordField::none(|v| valid_year(v, 1920, 2002)),
            iyr: InputRecordField::none(|v| valid_year(v, 2010, 2020)),
            eyr: InputRecordField::none(|v| valid_year(v, 2020, 2030)),
            hgt: InputRecordField::none(|v| valid_height(v, 59, 76, 150, 193)),
            hcl: InputRecordField::none(|v| valid_color(v)),
            ecl: InputRecordField::none(|v| valid_eye_color(v)),
            pid: InputRecordField::none(|v| valid_passport_id(v)),
            cid: InputRecordField::none(|_| true),
        }
    }

    fn indicates_new_record(line: &String) -> bool {
        line.is_empty()
    }

    fn parse(&mut self, line: &String) -> Result<(), Self::Err> {
        if !line.is_empty() {
            for tuple in line.split(' ') {
                match tuple.find(':') {
                    Some(idx) => {
                        let key = &tuple[..idx];
                        let value = tuple[idx + 1..].to_string();

                        match key {
                            "byr" => self.byr.set_value(value),
                            "iyr" => self.iyr.set_value(value),
                            "eyr" => self.eyr.set_value(value),
                            "hgt" => self.hgt.set_value(value),
                            "hcl" => self.hcl.set_value(value),
                            "ecl" => self.ecl.set_value(value),
                            "pid" => self.pid.set_value(value),
                            "cid" => self.cid.set_value(value),
                            _ => {
                                return Err(InputRecordError {
                                    msg: format!("Invalid key '{}' in line '{}'", key, line),
                                })
                            }
                        }
                    }
                    None => {
                        return Err(InputRecordError {
                            msg: format!("Invalid tuple '{}' in line '{}'", tuple, line),
                        })
                    }
                }
            }
        }

        Ok(())
    }
}
