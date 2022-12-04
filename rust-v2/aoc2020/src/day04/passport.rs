use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Default)]
pub struct Passport {
    pub byr: Option<String>,
    pub iyr: Option<String>,
    pub eyr: Option<String>,
    pub hgt: Option<String>,
    pub hcl: Option<String>,
    pub ecl: Option<String>,
    pub pid: Option<String>,
    pub cid: Option<String>,
}

impl Passport {
    pub fn is_complete(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    pub fn is_valid(&self) -> bool {
        self.is_complete()
            && is_valid_year(self.byr.as_ref().unwrap(), 1920, 2002)
            && is_valid_year(self.iyr.as_ref().unwrap(), 2010, 2020)
            && is_valid_year(self.eyr.as_ref().unwrap(), 2020, 2030)
            && is_valid_height(self.hgt.as_ref().unwrap(), 150, 193, 59, 76)
            && is_valid_hair_color(self.hcl.as_ref().unwrap())
            && is_valid_eye_color(self.ecl.as_ref().unwrap())
            && is_valid_pid(self.pid.as_ref().unwrap())
    }
}

fn is_valid_year(inp: &str, min: u32, max: u32) -> bool {
    match inp.parse::<u32>() {
        Ok(v) => v >= min && v <= max,
        Err(_) => false,
    }
}

fn is_valid_height(inp: &str, min_cm: u32, max_cm: u32, min_in: u32, max_in: u32) -> bool {
    match inp.strip_suffix("cm") {
        Some(v) => match v.parse::<u32>() {
            Ok(v) => v >= min_cm && v <= max_cm,
            Err(_) => false,
        },
        None => match inp.strip_suffix("in") {
            Some(v) => match v.parse::<u32>() {
                Ok(v) => v >= min_in && v <= max_in,
                Err(_) => false,
            },
            None => false,
        },
    }
}

fn is_valid_hair_color(inp: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    }
    RE.is_match(inp)
}

fn is_valid_eye_color(inp: &str) -> bool {
    inp == "amb"
        || inp == "blu"
        || inp == "brn"
        || inp == "gry"
        || inp == "grn"
        || inp == "hzl"
        || inp == "oth"
}

fn is_valid_pid(inp: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\d{9}$").unwrap();
    }
    RE.is_match(inp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_year() {
        assert!(is_valid_year("2020", 2010, 2030));
        assert!(is_valid_year("2020", 2020, 2030));
        assert!(is_valid_year("2020", 2010, 2020));
        assert!(!is_valid_year("2020", 2021, 2030));
        assert!(!is_valid_year("2020", 2010, 2019));
        assert!(!is_valid_year("garbage", 2010, 2030));
    }

    #[test]
    fn test_is_valid_height() {
        assert!(is_valid_height("10cm", 5, 15, 5, 15));
        assert!(is_valid_height("10cm", 10, 15, 5, 15));
        assert!(is_valid_height("10cm", 5, 10, 5, 15));
        assert!(is_valid_height("10in", 5, 15, 5, 15));
        assert!(is_valid_height("10in", 5, 15, 10, 15));
        assert!(is_valid_height("10in", 5, 15, 5, 10));
        assert!(!is_valid_height("10cm", 11, 15, 5, 15));
        assert!(!is_valid_height("10cm", 5, 9, 5, 15));
        assert!(!is_valid_height("10in", 5, 15, 11, 15));
        assert!(!is_valid_height("10in", 5, 15, 5, 9));
        assert!(!is_valid_height("10mm", 5, 15, 5, 15));
        assert!(!is_valid_height("10", 5, 15, 5, 15));
        assert!(!is_valid_height("garbage", 5, 15, 5, 15));
    }

    #[test]
    fn test_is_valid_hair_color() {
        assert!(is_valid_hair_color("#000000"));
        assert!(is_valid_hair_color("#ffffff"));
        assert!(is_valid_hair_color("#0369be"));
        assert!(!is_valid_hair_color("#000"));
        assert!(!is_valid_hair_color("000000"));
        assert!(!is_valid_hair_color("garbage"));
    }

    #[test]
    fn test_is_valid_eye_color() {
        assert!(is_valid_eye_color("amb"));
        assert!(is_valid_eye_color("blu"));
        assert!(is_valid_eye_color("brn"));
        assert!(is_valid_eye_color("gry"));
        assert!(is_valid_eye_color("grn"));
        assert!(is_valid_eye_color("hzl"));
        assert!(is_valid_eye_color("oth"));
        assert!(!is_valid_eye_color(""));
    }

    #[test]
    fn test_is_valid_pid() {
        assert!(is_valid_pid("123456789"));
        assert!(is_valid_pid("000123456"));
        assert!(!is_valid_pid("12345678"));
        assert!(!is_valid_pid("1234567890"));
        assert!(!is_valid_pid("garbage!!"));
    }
}
