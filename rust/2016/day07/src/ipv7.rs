use std::str::FromStr;

use rdcl_aoc_helpers::error::ParseError;

#[derive(Debug)]
enum Sequence {
    Super(String),
    Hyper(String),
}

impl Sequence {
    fn is_super(&self) -> bool {
        matches!(self, Sequence::Super(_))
    }

    fn is_hyper(&self) -> bool {
        matches!(self, Sequence::Hyper(_))
    }

    fn get_seq(&self) -> String {
        match self {
            Sequence::Super(v) => v.to_string(),
            Sequence::Hyper(v) => v.to_string(),
        }
    }

    fn has_abba(&self) -> bool {
        let seq = self.get_seq();

        for (idx, ch1) in seq.chars().enumerate().take(seq.len() - 3) {
            let ch2 = seq.chars().nth(idx + 1).unwrap();
            let ch3 = seq.chars().nth(idx + 2).unwrap();
            let ch4 = seq.chars().nth(idx + 3).unwrap();

            if ch1 != ch2 && ch1 == ch4 && ch2 == ch3 {
                return true;
            }
        }

        false
    }

    fn get_abas(&self) -> Vec<(char, char)> {
        match self {
            Sequence::Super(seq) => {
                let mut abas = Vec::new();
                for (idx, ch1) in seq.chars().enumerate().take(seq.len() - 2) {
                    let ch2 = seq.chars().nth(idx + 1).unwrap();
                    let ch3 = seq.chars().nth(idx + 2).unwrap();

                    if ch1 == ch3 && ch1 != ch2 {
                        abas.push((ch1, ch2));
                    }
                }
                abas
            }
            Sequence::Hyper(_) => Vec::new(),
        }
    }

    fn contains_bab(&self, (a, b): (char, char)) -> bool {
        let mut bab = String::new();
        bab.push(b);
        bab.push(a);
        bab.push(b);
        match self {
            Sequence::Super(_) => false,
            Sequence::Hyper(seq) => seq.contains(&bab),
        }
    }
}

#[derive(Debug)]
pub struct IPv7 {
    sequences: Vec<Sequence>,
}

impl IPv7 {
    pub fn supports_tls(&self) -> bool {
        let mut supports_tls = false;
        for seq in &self.sequences {
            if seq.is_hyper() && seq.has_abba() {
                return false;
            }

            if !supports_tls && seq.is_super() && seq.has_abba() {
                supports_tls = true;
            }
        }

        supports_tls
    }

    pub fn supports_ssl(&self) -> bool {
        let abas: Vec<(char, char)> = self.sequences.iter().flat_map(Sequence::get_abas).collect();

        for (a, b) in abas {
            if self.sequences.iter().any(|seq| seq.contains_bab((a, b))) {
                return true;
            }
        }

        false
    }
}

impl FromStr for IPv7 {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sequences = Vec::new();
        let mut offset = 0;

        while let Some((idx1, idx2)) = find_sub_seq(s, offset) {
            sequences.push(Sequence::Super(s[offset..idx1].to_string()));
            sequences.push(Sequence::Hyper(s[idx1 + 1..idx2].to_string()));
            offset = idx2 + 1;
        }
        sequences.push(Sequence::Super(s[offset..].to_string()));

        Ok(IPv7 { sequences })
    }
}

fn find_sub_seq(s: &str, offset: usize) -> Option<(usize, usize)> {
    if let (Some(idx1), Some(idx2)) = (s[offset..].find('['), s[offset..].find(']')) {
        Some((offset + idx1, offset + idx2))
    } else {
        None
    }
}
