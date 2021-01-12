use std::collections::VecDeque;
use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::input::WithReadLines;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let masses: Vec<u32> = File::open(&args[1]).read_lines(1).collect();

    println!(
        "The total amount of required fuel is {}.",
        masses.iter().map(|mass| compute_fuel(*mass)).sum::<u32>()
    );
    println!(
        "Also accounting for the mass of the fuel itself, the total amount of required fuel is {}.",
        tyranny(&masses)
    );
}

fn compute_fuel(mass: u32) -> u32 {
    (mass / 3).saturating_sub(2)
}

fn tyranny(masses: &[u32]) -> u32 {
    let mut total_mass = 0;
    let mut queue: VecDeque<u32> = VecDeque::new();

    for &mass in masses {
        queue.push_back(mass);
    }

    while let Some(mass) = queue.pop_front() {
        let fuel_mass = compute_fuel(mass);
        if fuel_mass > 0 {
            total_mass += fuel_mass;
            queue.push_back(fuel_mass);
        }
    }

    total_mass
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_fuel() {
        assert_eq!(compute_fuel(12), 2);
        assert_eq!(compute_fuel(14), 2);
        assert_eq!(compute_fuel(1969), 654);
        assert_eq!(compute_fuel(100756), 33583);
    }

    #[test]
    fn test_tyranny() {
        assert_eq!(tyranny(&[14]), 2);
        assert_eq!(tyranny(&[1969]), 966);
        assert_eq!(tyranny(&[100756]), 50346);
    }
}
