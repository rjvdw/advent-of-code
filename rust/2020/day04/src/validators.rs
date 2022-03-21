use regex::Regex;

pub fn valid_year(v: &str, lower_bound: i32, upper_bound: i32) -> bool {
    match v.parse::<i32>() {
        Ok(value) => lower_bound <= value && value <= upper_bound,
        Err(_) => false,
    }
}

pub fn valid_height(
    v: &str,
    lower_bound_in: i32,
    upper_bound_in: i32,
    lower_bound_cm: i32,
    upper_bound_cm: i32,
) -> bool {
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
        Err(_) => false,
    }
}

pub fn valid_color(v: &str) -> bool {
    Regex::new(r"^#[0-9a-f]{6}$").unwrap().is_match(v)
}

pub fn valid_eye_color(v: &str) -> bool {
    v == "amb" || v == "blu" || v == "brn" || v == "gry" || v == "grn" || v == "hzl" || v == "oth"
}

pub fn valid_passport_id(v: &str) -> bool {
    Regex::new(r"^\d{9}$").unwrap().is_match(v)
}

#[cfg(test)]
mod valid_year_tests {
    use super::*;

    #[test]
    fn test_valid_year_between_bounds() {
        assert!(valid_year("2020", 2000, 2030))
    }

    #[test]
    fn test_valid_year_at_lower_bound() {
        assert!(valid_year("2000", 2000, 2030))
    }

    #[test]
    fn test_valid_year_at_upper_bound() {
        assert!(valid_year("2030", 2000, 2030))
    }

    #[test]
    fn test_valid_year_below_lower_bound() {
        assert!(!valid_year("1999", 2000, 2030))
    }

    #[test]
    fn test_valid_year_above_upper_bound() {
        assert!(!valid_year("2031", 2000, 2030))
    }

    #[test]
    fn test_invalid_year() {
        assert!(!valid_year("abcd", 2000, 2030))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod valid_height_tests {
        use super::*;

        #[test]
        fn test_valid_height_between_bounds_using_cm() {
            assert!(valid_height("175cm", 50, 100, 100, 200))
        }

        #[test]
        fn test_valid_height_at_lower_bound_using_cm() {
            assert!(valid_height("100cm", 50, 100, 100, 200))
        }

        #[test]
        fn test_valid_height_at_upper_bound_using_cm() {
            assert!(valid_height("200cm", 50, 100, 100, 200))
        }

        #[test]
        fn test_valid_height_below_lower_bound_using_cm() {
            assert!(!valid_height("99cm", 50, 100, 100, 200))
        }

        #[test]
        fn test_valid_height_above_upper_bound_using_cm() {
            assert!(!valid_height("201cm", 50, 100, 100, 200))
        }

        #[test]
        fn test_valid_height_between_bounds_using_in() {
            assert!(valid_height("75in", 50, 100, 100, 200))
        }

        #[test]
        fn test_valid_height_at_lower_bound_using_in() {
            assert!(valid_height("50in", 50, 100, 100, 200))
        }

        #[test]
        fn test_valid_height_at_upper_bound_using_in() {
            assert!(valid_height("100in", 50, 100, 100, 200))
        }

        #[test]
        fn test_valid_height_below_lower_bound_using_in() {
            assert!(!valid_height("49in", 50, 100, 100, 200))
        }

        #[test]
        fn test_valid_height_above_upper_bound_using_in() {
            assert!(!valid_height("101in", 50, 100, 100, 200))
        }

        #[test]
        fn test_invalid_unit() {
            assert!(!valid_height("150km", 50, 100, 100, 200))
        }

        #[test]
        fn test_invalid_height() {
            assert!(!valid_height("abcm", 50, 100, 100, 200))
        }
    }

    #[cfg(test)]
    mod valid_color_tests {
        use super::*;

        #[test]
        fn test_valid_color() {
            assert!(valid_color("#18ab2e"))
        }

        #[test]
        fn test_invalid_color() {
            assert!(!valid_color("#qwerty"));
            assert!(!valid_color("#123"));
            assert!(!valid_color("123123"));
        }
    }

    #[cfg(test)]
    mod valid_eye_color_tests {
        use super::*;

        #[test]
        fn test_valid_color() {
            assert!(valid_eye_color("amb"));
            assert!(valid_eye_color("blu"));
            assert!(valid_eye_color("brn"));
            assert!(valid_eye_color("gry"));
            assert!(valid_eye_color("grn"));
            assert!(valid_eye_color("hzl"));
            assert!(valid_eye_color("oth"));
        }

        #[test]
        fn test_invalid_color() {
            assert!(!valid_eye_color("ylw"))
        }
    }

    #[cfg(test)]
    mod valid_passport_id_tests {
        use super::*;

        #[test]
        fn test_valid_passport_id() {
            assert!(valid_passport_id("012345678"))
        }

        #[test]
        fn test_invalid_passport_id() {
            assert!(!valid_passport_id("12345678"));
            assert!(!valid_passport_id("0123456789"));
            assert!(!valid_passport_id("abcdefghi"));
        }
    }
}
