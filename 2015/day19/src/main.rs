use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::{ParseError, WithOrExit};

fn main() {
    let args = get_args(&["<input file>"], 1);
    let (molecule, replacements) = parse_input(&args[1]).or_exit_with(1);

    let molecules = apply_replacements(&molecule, &replacements);
    println!(
        "After applying the replacements once, there are {} possible combinations.",
        molecules.len()
    );

    let steps = count_steps(&molecule);
    println!("It takes {} steps to create the medicine.", steps);
}

fn apply_replacements(molecule: &[Atom], replacements: &[(Atom, Molecule)]) -> HashSet<Molecule> {
    let mut result = HashSet::new();
    for (left, right) in replacements {
        for (idx, atom) in molecule.iter().enumerate() {
            if atom.eq(left) {
                let len = molecule.len() + right.len() - 1;
                let mut modified = Vec::with_capacity(len);
                modified.extend_from_slice(&molecule[..idx]);
                modified.extend(right.to_vec());
                modified.extend_from_slice(&molecule[idx + 1..]);
                result.insert(modified);
            }
        }
    }
    result
}

fn count_steps(molecule: &[Atom]) -> usize {
    // https://www.reddit.com/r/adventofcode/comments/3xflz8/day_19_solutions/cy4etju
    let elements = molecule.len();
    let parentheses = molecule
        .iter()
        .filter(|a| parse_atom("Rn").eq(a) || parse_atom("Ar").eq(a))
        .count();
    let commas = molecule.iter().filter(|a| parse_atom("Y").eq(a)).count();

    elements - parentheses - 2 * commas - 1
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Atom(String);

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

type Molecule = Vec<Atom>;

fn parse_input(path: &str) -> Result<(Molecule, Vec<(Atom, Molecule)>), ParseError> {
    let file = File::open(path)?;
    let mut molecule = Default::default();
    let mut replacements = Vec::new();
    let mut parsing_replacements = true;
    for line in BufReader::new(file).lines() {
        let line = line?;
        if line.is_empty() {
            parsing_replacements = false;
        } else if parsing_replacements {
            if let Some(idx) = line.find("=>") {
                let left = Atom(line[..idx].trim().to_string());
                let right = parse_molecule(line[idx + 2..].trim());
                replacements.push((left, right));
            } else {
                return Err(ParseError(format!("Invalid input line: {}", line)));
            }
        } else {
            molecule = parse_molecule(&line);
        }
    }
    Ok((molecule, replacements))
}

fn parse_molecule(molecule: &str) -> Vec<Atom> {
    let mut parsed = Vec::new();
    let mut offset = 0;
    for (idx, char) in molecule.chars().enumerate().skip(1) {
        if char.is_uppercase() {
            parsed.push(Atom(molecule[offset..idx].to_string()));
            offset = idx;
        }
    }
    parsed.push(Atom(molecule[offset..].to_string()));
    parsed
}

fn parse_atom(atom: &str) -> Atom {
    Atom(atom.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_molecule_1() {
        let molecule = "MgHOHArHMgOHCaCaCaC";

        let parsed = parse_molecule(molecule);

        assert_eq!(
            parsed,
            vec![
                parse_atom("Mg"),
                parse_atom("H"),
                parse_atom("O"),
                parse_atom("H"),
                parse_atom("Ar"),
                parse_atom("H"),
                parse_atom("Mg"),
                parse_atom("O"),
                parse_atom("H"),
                parse_atom("Ca"),
                parse_atom("Ca"),
                parse_atom("Ca"),
                parse_atom("C"),
            ]
        );
    }

    #[test]
    fn test_parse_molecule_2() {
        let molecule = "MgCa";

        let parsed = parse_molecule(molecule);

        assert_eq!(parsed, vec![parse_atom("Mg"), parse_atom("Ca")]);
    }

    #[test]
    fn test_parse_molecule_3() {
        let molecule = "Mg";

        let parsed = parse_molecule(molecule);

        assert_eq!(parsed, vec![parse_atom("Mg")]);
    }

    #[test]
    fn test_parse_molecule_4() {
        let molecule = "C";

        let parsed = parse_molecule(molecule);

        assert_eq!(parsed, vec![parse_atom("C")]);
    }

    #[test]
    fn test_apply_replacements_1() {
        let molecule = vec![parse_atom("H"), parse_atom("O"), parse_atom("H")];
        let replacements = get_replacements();

        let molecules = apply_replacements(&molecule, &replacements);

        assert_eq!(molecules.len(), 4);
        assert!(molecules.contains(&parse_molecule("HOOH")));
        assert!(molecules.contains(&parse_molecule("HOHO")));
        assert!(molecules.contains(&parse_molecule("OHOH")));
        assert!(molecules.contains(&parse_molecule("HHHH")));
    }

    #[test]
    fn test_apply_replacements_2() {
        let molecule = vec![
            parse_atom("H"),
            parse_atom("O"),
            parse_atom("H"),
            parse_atom("O"),
            parse_atom("H"),
            parse_atom("O"),
        ];
        let replacements = get_replacements();

        let molecules = apply_replacements(&molecule, &replacements);

        assert_eq!(molecules.len(), 7);
    }

    fn get_replacements() -> Vec<(Atom, Molecule)> {
        vec![
            (parse_atom("e"), parse_molecule("H")),
            (parse_atom("e"), parse_molecule("O")),
            (parse_atom("H"), parse_molecule("HO")),
            (parse_atom("H"), parse_molecule("OH")),
            (parse_atom("O"), parse_molecule("HH")),
        ]
    }
}
