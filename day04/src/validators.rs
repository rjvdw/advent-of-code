use regex::Regex;

pub fn valid_year(v: &String, lower_bound: i32, upper_bound: i32) -> bool {
    match v.parse::<i32>() {
        Ok(value) => lower_bound <= value && value <= upper_bound,
        Err(_) => false,
    }
}

pub fn valid_height(v: &String, lower_bound_in: i32, upper_bound_in: i32, lower_bound_cm: i32, upper_bound_cm: i32) -> bool {
    let unit_idx = v.len() - 2;
    match v[..unit_idx].parse::<i32>() {
        Ok(value) => {
            let unit = &v[unit_idx..];

            match unit {
                "in" => lower_bound_in <= value && value <= upper_bound_in,
                "cm" => lower_bound_cm <= value && value <= upper_bound_cm,
                _ => false,
            }
        }
        Err(_) => false
    }
}

pub fn valid_color(v: &String) -> bool {
    Regex::new(r"^#[0-9a-f]{6}$").unwrap().is_match(v.as_str())
}

pub fn valid_eye_color(v: &String) -> bool {
    v == "amb" || v == "blu" || v == "brn" || v == "gry" || v == "grn" || v == "hzl" || v == "oth"
}

pub fn valid_passport_id(v: &String) -> bool {
    Regex::new(r"^\d{9}$").unwrap().is_match(v.as_str())
}

#[cfg(test)]
mod valid_year_tests {
    use super::*;

    #[test]
    fn test_valid_year_between_bounds() {
        assert!(valid_year(&"2020".to_string(), 2000, 2030))
    }

    #[test]
    fn test_valid_year_at_lower_bound() {
        assert!(valid_year(&"2000".to_string(), 2000, 2030))
    }

    #[test]
    fn test_valid_year_at_upper_bound() {
        assert!(valid_year(&"2030".to_string(), 2000, 2030))
    }

    #[test]
    fn test_valid_year_below_lower_bound() {
        assert!(!valid_year(&"1999".to_string(), 2000, 2030))
    }

    #[test]
    fn test_valid_year_above_upper_bound() {
        assert!(!valid_year(&"2031".to_string(), 2000, 2030))
    }

    #[test]
    fn test_invalid_year() {
        assert!(!valid_year(&"abcd".to_string(), 2000, 2030))
    }
}

#[cfg(test)]
mod valid_height_tests {
    use super::*;

    #[test]
    fn test_valid_height_between_bounds_using_cm() {
        assert!(valid_height(&"175cm".to_string(), 50, 100, 100, 200))
    }

    #[test]
    fn test_valid_height_at_lower_bound_using_cm() {
        assert!(valid_height(&"100cm".to_string(), 50, 100, 100, 200))
    }

    #[test]
    fn test_valid_height_at_upper_bound_using_cm() {
        assert!(valid_height(&"200cm".to_string(), 50, 100, 100, 200))
    }

    #[test]
    fn test_valid_height_below_lower_bound_using_cm() {
        assert!(!valid_height(&"99cm".to_string(), 50, 100, 100, 200))
    }

    #[test]
    fn test_valid_height_above_upper_bound_using_cm() {
        assert!(!valid_height(&"201cm".to_string(), 50, 100, 100, 200))
    }

    #[test]
    fn test_valid_height_between_bounds_using_in() {
        assert!(valid_height(&"75in".to_string(), 50, 100, 100, 200))
    }

    #[test]
    fn test_valid_height_at_lower_bound_using_in() {
        assert!(valid_height(&"50in".to_string(), 50, 100, 100, 200))
    }

    #[test]
    fn test_valid_height_at_upper_bound_using_in() {
        assert!(valid_height(&"100in".to_string(), 50, 100, 100, 200))
    }

    #[test]
    fn test_valid_height_below_lower_bound_using_in() {
        assert!(!valid_height(&"49in".to_string(), 50, 100, 100, 200))
    }

    #[test]
    fn test_valid_height_above_upper_bound_using_in() {
        assert!(!valid_height(&"101in".to_string(), 50, 100, 100, 200))
    }

    #[test]
    fn test_invalid_unit() {
        assert!(!valid_height(&"150km".to_string(), 50, 100, 100, 200))
    }

    #[test]
    fn test_invalid_height() {
        assert!(!valid_height(&"abcm".to_string(), 50, 100, 100, 200))
    }
}

#[cfg(test)]
mod valid_color_tests {
    use super::*;

    #[test]
    fn test_valid_color() {
        assert!(valid_color(&"#18ab2e".to_string()))
    }

    #[test]
    fn test_invalid_color() {
        assert!(!valid_color(&"#qwerty".to_string()));
        assert!(!valid_color(&"#123".to_string()));
        assert!(!valid_color(&"123123".to_string()));
    }
}

#[cfg(test)]
mod valid_eye_color_tests {
    use super::*;

    #[test]
    fn test_valid_color() {
        assert!(valid_eye_color(&"amb".to_string()));
        assert!(valid_eye_color(&"blu".to_string()));
        assert!(valid_eye_color(&"brn".to_string()));
        assert!(valid_eye_color(&"gry".to_string()));
        assert!(valid_eye_color(&"grn".to_string()));
        assert!(valid_eye_color(&"hzl".to_string()));
        assert!(valid_eye_color(&"oth".to_string()));
    }

    #[test]
    fn test_invalid_color() {
        assert!(!valid_eye_color(&"ylw".to_string()))
    }
}

#[cfg(test)]
mod valid_passport_id_tests {
    use super::*;

    #[test]
    fn test_valid_passport_id() {
        assert!(valid_passport_id(&"012345678".to_string()))
    }

    #[test]
    fn test_invalid_passport_id() {
        assert!(!valid_passport_id(&"12345678".to_string()));
        assert!(!valid_passport_id(&"0123456789".to_string()));
        assert!(!valid_passport_id(&"abcdefghi".to_string()));
    }
}
