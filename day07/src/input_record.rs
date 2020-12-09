use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

const INPUT_SEPARATOR: &str = " bags contain";

#[derive(Debug)]
pub struct InputRecord {
    pub color: String,
    pub contains: HashMap<String, u32>,
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

impl FromStr for InputRecord {
    type Err = InputRecordError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let error = || {
            Err(InputRecordError {
                msg: format!("Invalid input line: '{}'", s),
            })
        };
        match s.find(INPUT_SEPARATOR) {
            Some(pos) => {
                let color = s[..pos].to_string();
                let mut contains = HashMap::new();

                for ss in s[pos + INPUT_SEPARATOR.len()..].split(",") {
                    let ss = ss.trim();
                    if ss != "no other bags." {
                        let idx1 = ss.find(' ');
                        if idx1.is_none() {
                            return error();
                        }
                        let idx1 = idx1.unwrap();

                        let idx2 = ss.rfind(' ');
                        if idx2.is_none() {
                            return error();
                        }
                        let idx2 = idx2.unwrap();

                        let count = match ss[..idx1].parse::<u32>() {
                            Ok(v) => v,
                            Err(_) => return error(),
                        };
                        let color = ss[idx1 + 1..idx2].to_string();

                        contains.insert(color, count);
                    }
                }

                Ok(InputRecord { color, contains })
            }
            None => error(),
        }
    }
}

#[cfg(test)]
mod tests {
    use helpers::parse_input;

    use super::*;

    #[test]
    fn test() {
        let values = parse_input::<InputRecord>(vec![
            "light red bags contain 1 bright white bag, 2 muted yellow bags.",
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
            "bright white bags contain 1 shiny gold bag.",
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
            "faded blue bags contain no other bags.",
            "dotted black bags contain no other bags.",
        ])
        .unwrap();

        assert_eq!(values[0].color, "light red");
        assert_eq!(values[0].contains.len(), 2);
        assert_eq!(values[0].contains["bright white"], 1);
        assert_eq!(values[0].contains["muted yellow"], 2);

        assert_eq!(values[1].color, "dark orange");
        assert_eq!(values[1].contains.len(), 2);
        assert_eq!(values[1].contains["bright white"], 3);
        assert_eq!(values[1].contains["muted yellow"], 4);

        assert_eq!(values[2].color, "bright white");
        assert_eq!(values[2].contains.len(), 1);
        assert_eq!(values[2].contains["shiny gold"], 1);

        assert_eq!(values[3].color, "muted yellow");
        assert_eq!(values[3].contains.len(), 2);
        assert_eq!(values[3].contains["shiny gold"], 2);
        assert_eq!(values[3].contains["faded blue"], 9);

        assert_eq!(values[4].color, "shiny gold");
        assert_eq!(values[4].contains.len(), 2);
        assert_eq!(values[4].contains["dark olive"], 1);
        assert_eq!(values[4].contains["vibrant plum"], 2);

        assert_eq!(values[5].color, "dark olive");
        assert_eq!(values[5].contains.len(), 2);
        assert_eq!(values[5].contains["faded blue"], 3);
        assert_eq!(values[5].contains["dotted black"], 4);

        assert_eq!(values[6].color, "vibrant plum");
        assert_eq!(values[6].contains.len(), 2);
        assert_eq!(values[6].contains["faded blue"], 5);
        assert_eq!(values[6].contains["dotted black"], 6);

        assert_eq!(values[7].color, "faded blue");
        assert_eq!(values[7].contains.len(), 0);

        assert_eq!(values[8].color, "dotted black");
        assert_eq!(values[8].contains.len(), 0);
    }
}
