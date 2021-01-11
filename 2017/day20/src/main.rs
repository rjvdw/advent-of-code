use std::collections::HashMap;
use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::input::WithReadLines;
use rdcl_aoc_helpers::math::taxi_cab_3d;

use crate::particle::Particle;

mod particle;

const ORIGIN: (i64, i64, i64) = (0, 0, 0);

fn main() {
    let args = get_args(&["<input file>"], 1);
    let particles: Vec<Particle> = File::open(&args[1]).read_lines(1).collect();

    let idx = find_long_term_closest(&particles);
    println!(
        "In the long term, particle {} stays the closest to the origin.",
        idx
    );

    let count = resolve_collisions(&particles);
    println!(
        "After resolving all the collisions, there are {} particles remaining.",
        count
    );
}

fn find_long_term_closest(particles: &[Particle]) -> usize {
    particles
        .iter()
        // TODO: Don't hardcode this number
        .map(|particle| particle.position_at_time(1_000_000))
        .enumerate()
        .min_by_key(|(_, p)| taxi_cab_3d(*p, ORIGIN))
        .map(|(idx, _)| idx)
        .unwrap()
}

fn resolve_collisions(particles: &[Particle]) -> usize {
    let mut particles = particles.to_vec();

    for _ in 1..=100 {
        let mut seen: HashMap<(i64, i64, i64), Vec<usize>> = HashMap::new();

        for (idx, particle) in particles.iter_mut().enumerate() {
            let p = particle.next().unwrap();
            seen.entry(p).or_insert_with(Vec::new).push(idx)
        }

        particles = seen
            .values()
            .filter(|v| v.len() == 1)
            .map(|v| v.first().unwrap())
            .map(|idx| particles[*idx])
            .collect();
    }

    particles.len()
}

#[cfg(test)]
mod tests {
    use rdcl_aoc_helpers::input::WithAsRecords;

    use super::*;

    #[test]
    fn test_find_long_term_closest() {
        let particles = vec![
            "p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>",
            "p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>",
        ]
        .as_records::<Particle>()
        .unwrap();

        assert_eq!(find_long_term_closest(&particles), 0);
    }

    #[test]
    fn test_resolve_collisions() {
        let particles = vec![
            "p=<-6,0,0>, v=< 3,0,0>, a=< 0,0,0>",
            "p=<-4,0,0>, v=< 2,0,0>, a=< 0,0,0>",
            "p=<-2,0,0>, v=< 1,0,0>, a=< 0,0,0>",
            "p=< 3,0,0>, v=<-1,0,0>, a=< 0,0,0>",
        ]
        .as_records::<Particle>()
        .unwrap();

        assert_eq!(resolve_collisions(&particles), 1);
    }
}
