use std::collections::HashMap;
use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;
use rdcl_aoc_helpers::input::WithReadLines;

use crate::deer::Deer;

mod deer;

fn main() {
    let args = get_args(&["<input file>", "<duration>"], 1);
    let mut deer = File::open(&args[1]).read_lines(1).collect::<Vec<Deer>>();
    let duration = args[2].parse::<u32>().or_exit_with(1);

    let ((winner, distance), scores) = travel(&mut deer, duration);
    println!(
        "After {} seconds, {} has won and traveled {} km.",
        duration, winner, distance
    );
    let (deer, score) = scores.iter().max_by_key(|(_, v)| *v).unwrap();
    println!(
        "However, {} has the highest score, with {} points.",
        deer, score
    );
}

fn travel(deer: &mut [Deer], duration: u32) -> ((String, u32), HashMap<String, u32>) {
    let mut winning_deer = (String::new(), 0);
    let mut scores: HashMap<String, u32> = HashMap::new();
    for _ in 0..duration {
        let mut winning_distance = 0;
        for d in &mut *deer {
            d.travel(1);
            if d.get_distance() > winning_distance {
                winning_distance = d.get_distance();
            }
        }
        for d in &mut *deer {
            if d.get_distance() == winning_distance {
                *scores.entry(d.get_name()).or_insert(0) += 1;
            }
        }
    }
    for d in deer {
        if d.get_distance() > winning_deer.1 {
            winning_deer = (d.get_name(), d.get_distance());
        }
    }
    (winning_deer, scores)
}

#[cfg(test)]
mod tests {
    use rdcl_aoc_helpers::input::WithAsRecords;

    use super::*;

    #[test]
    fn test() {
        let mut deer = vec![
            "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.",
            "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.",
        ]
        .as_records()
        .unwrap();

        let (_, scores) = travel(&mut deer, 1000);

        assert_eq!(deer[0].get_distance(), 1120);
        assert_eq!(deer[1].get_distance(), 1056);
        assert_eq!(*scores.get(&deer[0].get_name()).unwrap(), 312);
        assert_eq!(*scores.get(&deer[1].get_name()).unwrap(), 689);
    }
}
