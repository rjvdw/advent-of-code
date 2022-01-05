use std::collections::HashMap;
use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::input::WithReadLines;
use rdcl_aoc_helpers::math::lcm;

use moon::Moon;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let moons: Vec<Moon> = File::open(&args[1]).read_lines(1).collect();

    println!(
        "After 1000 steps, the total energy in the system is {}.",
        run_simulation(&moons, 1000)
    );

    println!(
        "After {} steps, the simulation enters a position that was previously seen.",
        find_loop(&moons)
    )
}

fn run_simulation(moons: &[Moon], steps: usize) -> i64 {
    let mut moons = moons.to_vec();
    for _ in 1..=steps {
        moons = moons.iter().map(|moon| moon.update_moon(&moons)).collect();
    }
    moons.iter().map(Moon::total_energy).sum()
}

macro_rules! get_seen_key {
    ($moons:expr, $x:tt) => {
        $moons
            .iter()
            .map(|moon| {
                let position = moon.get_position();
                let velocity = moon.get_velocity();
                (position.$x, velocity.$x)
            })
            .collect::<Vec<(i64, i64)>>()
    };
}

macro_rules! update_period {
    ($period:expr, $seen:expr, $moons:expr, $t:expr, $x:tt) => {
        if $period.is_none() {
            let key = get_seen_key!($moons, $x);
            if let Some(t_prev) = $seen.get(&key) {
                $period = Some((*t_prev, $t));
            } else {
                $seen.insert(key, $t);
            }
        }
    };
}

fn find_loop(moons: &[Moon]) -> usize {
    let mut moons = moons.to_vec();

    let mut seen_x: HashMap<Vec<(i64, i64)>, usize> = HashMap::new();
    let mut seen_y: HashMap<Vec<(i64, i64)>, usize> = HashMap::new();
    let mut seen_z: HashMap<Vec<(i64, i64)>, usize> = HashMap::new();

    seen_x.insert(get_seen_key!(moons, 0), 0);
    seen_y.insert(get_seen_key!(moons, 1), 0);
    seen_z.insert(get_seen_key!(moons, 2), 0);

    let mut period_x: Option<(usize, usize)> = None;
    let mut period_y: Option<(usize, usize)> = None;
    let mut period_z: Option<(usize, usize)> = None;

    for t in 1.. {
        moons = moons.iter().map(|moon| moon.update_moon(&moons)).collect();

        update_period!(period_x, seen_x, moons, t, 0);
        update_period!(period_y, seen_y, moons, t, 1);
        update_period!(period_z, seen_z, moons, t, 2);

        // TODO: These steps below will only actually work if `t_prev == 0`.
        if let (Some(px), Some(py), Some(pz)) = (period_x, period_y, period_z) {
            return lcm(lcm(px.1 - px.0, py.1 - py.0), pz.1 - pz.0);
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulation_100() {
        let moons = vec![
            Moon::new(-8, -10, 0),
            Moon::new(5, 5, 10),
            Moon::new(2, -7, 3),
            Moon::new(9, -8, -3),
        ];

        assert_eq!(run_simulation(&moons, 100), 1940);
    }

    #[test]
    fn test_find_loop_1() {
        let moons = vec![
            Moon::new(-1, 0, 2),
            Moon::new(2, -10, -7),
            Moon::new(4, -8, 8),
            Moon::new(3, 5, -1),
        ];

        assert_eq!(find_loop(&moons), 2772);
    }
}
