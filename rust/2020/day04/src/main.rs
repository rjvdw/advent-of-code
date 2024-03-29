extern crate rdcl_aoc_helpers;

use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::input::WithReadMultiLines;

use input_record::Passport;

mod input_record;
mod validators;

fn main() {
    let args = get_args(&["<input file>"], 1);

    let passports = File::open(&args[1]).read_multi_lines::<Passport>(1);
    let mut counts = (0, 0);
    for passport in passports {
        if passport.has_required_fields() {
            counts.0 += 1;
        }
        if passport.is_valid() {
            counts.1 += 1;
        }
    }

    println!("number of records with required fields: {}", counts.0);
    println!("number of valid records: {}", counts.1);
}

#[cfg(test)]
mod tests {
    use rdcl_aoc_helpers::input::WithAsMultilineRecords;

    use super::*;

    #[test]
    fn test_has_required_fields() {
        let values = vec![
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd",
            "byr:1937 iyr:2017 cid:147 hgt:183cm",
            "",
            "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884",
            "hcl:#cfa07d byr:1929",
            "",
            "hcl:#ae17e1 iyr:2013",
            "eyr:2024",
            "ecl:brn pid:760753108 byr:1931",
            "hgt:179cm",
            "",
            "hcl:#cfa07d eyr:2025 pid:166559648",
            "iyr:2011 ecl:brn hgt:59in",
        ]
        .as_multiline_records::<Passport>()
        .unwrap();

        assert!(values[0].has_required_fields());
        assert!(!values[1].has_required_fields());
        assert!(values[2].has_required_fields());
        assert!(!values[3].has_required_fields());
    }

    #[test]
    fn test_invalid_passports() {
        let values = vec![
            "eyr:1972 cid:100",
            "hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926",
            "",
            "iyr:2019",
            "hcl:#602927 eyr:1967 hgt:170cm",
            "ecl:grn pid:012533040 byr:1946",
            "",
            "hcl:dab227 iyr:2012",
            "ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277",
            "",
            "hgt:59cm ecl:zzz",
            "eyr:2038 hcl:74454a iyr:2023",
            "pid:3556412378 byr:2007",
        ]
        .as_multiline_records::<Passport>()
        .unwrap();

        assert!(!values[0].is_valid());
        assert!(!values[1].is_valid());
        assert!(!values[2].is_valid());
        assert!(!values[3].is_valid());
    }

    #[test]
    fn test_valid_passports() {
        let values = vec![
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980",
            "hcl:#623a2f",
            "",
            "eyr:2029 ecl:blu cid:129 byr:1989",
            "iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm",
            "",
            "hcl:#888785",
            "hgt:164cm byr:2001 iyr:2015 cid:88",
            "pid:545766238 ecl:hzl",
            "eyr:2022",
            "",
            "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719",
        ]
        .as_multiline_records::<Passport>()
        .unwrap();

        assert!(values[0].is_valid());
        assert!(values[1].is_valid());
        assert!(values[2].is_valid());
        assert!(values[3].is_valid());
    }
}
