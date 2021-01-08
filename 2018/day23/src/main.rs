use std::cmp::Ordering;
use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::input::WithReadLines;
use rdcl_aoc_helpers::math::taxi_cab_3d;

use crate::nanobot::Nanobot;
use crate::region::Region;

mod nanobot;
mod region;

pub type Point = (i64, i64, i64);

pub const ORIGIN: Point = (0, 0, 0);

fn main() {
    let args = get_args(&["<input file>"], 1);
    let nanobots = File::open(&args[1]).read_lines(1).collect::<Vec<Nanobot>>();

    match count_nr_nanobots_in_range_of_strongest(&nanobots) {
        Some(count) => {
            println!(
                "The strongest nanobot has {} nanobots within its range.",
                count
            )
        }
        None => eprintln!("Empty list provided."),
    }

    match find_optimal_position(&nanobots) {
        Some(position) => {
            println!(
                "You should position yourself at {:?}. This point has distance {} from {:?}",
                position,
                distance_to_origin(position),
                ORIGIN
            )
        }
        None => eprintln!("Empty list provided."),
    }
}

fn count_nr_nanobots_in_range_of_strongest(nanobots: &[Nanobot]) -> Option<usize> {
    nanobots
        .iter()
        .max_by_key(|nanobot| nanobot.radius)
        .map(|one| {
            nanobots
                .iter()
                .filter(|other| one.in_range(other.position))
                .count()
        })
}

fn find_optimal_position(nanobots: &[Nanobot]) -> Option<Point> {
    let mut lower_bound = (i64::MAX, i64::MAX, i64::MAX);
    let mut upper_bound = (i64::MIN, i64::MIN, i64::MIN);

    for nanobot in nanobots {
        update_bounds(&mut lower_bound.0, &mut upper_bound.0, nanobot.position.0);
        update_bounds(&mut lower_bound.1, &mut upper_bound.1, nanobot.position.1);
        update_bounds(&mut lower_bound.2, &mut upper_bound.2, nanobot.position.2);
    }

    explore_region(
        nanobots,
        &Region {
            from: lower_bound,
            to: upper_bound,
        },
    )
    .map(|(_, p)| p)
}

/// Find the optimal point within a region. A point is optimal if is within range of the maximum
/// number of nanobots, and if is it is not further from the origin than other points that are
/// within range of this same number of nanobots. This function returns None if it is impossible to
/// determine such a point (i.e. there are no points within range of a nanobot), or a tuple that has
/// the number of nanobots that the point is in range of and the point itself.
fn explore_region(nanobots: &[Nanobot], region: &Region) -> Option<(usize, Point)> {
    let sub_regions = region.split();
    let mut scores = vec![0; sub_regions.len()];
    let mut order = vec![0; sub_regions.len()];
    for (idx, sub_region) in sub_regions.iter().enumerate() {
        scores[idx] = sub_region.count_influencing_nanobots(nanobots);
        order[idx] = idx;
    }
    order.sort_unstable_by_key(|&idx| scores[idx]);

    let mut optimal: Option<(usize, Point)> = None;
    for &idx in order.iter().rev() {
        let score = scores[idx];
        let sub_region = sub_regions[idx];

        if score == 0 || score < optimal.map(|(o, _)| o).unwrap_or(0) {
            // If there are no nanobots that influence this region, or if the number of nanobots
            // than influence this region is less than the best score we've found so far, then we
            // can skip this region. Since we have sorted based on the score, this means we can skip
            // all remaining regions as well.
            break;
        }

        if sub_region.is_fully_contained(nanobots) {
            let point = sub_region.closest_point_to_origin();
            if is_better_result(optimal, (score, point)) {
                optimal = Some((score, point));
            }
        } else if sub_region.is_small_enough() {
            for point in sub_region.iter() {
                let score = nanobots.iter().filter(|b| b.in_range(point)).count();
                if is_better_result(optimal, (score, point)) {
                    optimal = Some((score, point));
                }
            }
        } else if let Some(result) = explore_region(nanobots, &sub_region) {
            if is_better_result(optimal, result) {
                optimal = Some(result);
            }
        }
    }
    optimal
}

pub fn distance_to_origin(point: Point) -> i64 {
    taxi_cab_3d(point, ORIGIN)
}

fn is_better_result(current: Option<(usize, Point)>, next: (usize, Point)) -> bool {
    match current {
        Some(current) => match current.0.cmp(&next.0) {
            Ordering::Less => true,
            Ordering::Equal => distance_to_origin(next.1) < distance_to_origin(current.1),
            Ordering::Greater => false,
        },
        None => true,
    }
}

fn update_bounds(min: &mut i64, max: &mut i64, v: i64) {
    if v < *min {
        *min = v;
    }
    if v > *max {
        *max = v;
    }
}

#[cfg(test)]
mod tests {
    use rdcl_aoc_helpers::input::WithAsRecords;

    use super::*;

    #[test]
    fn test_count_nr_nanobots_in_range_of_strongest() {
        let nanobots = vec![
            "pos=<0,0,0>, r=4",
            "pos=<1,0,0>, r=1",
            "pos=<4,0,0>, r=3",
            "pos=<0,2,0>, r=1",
            "pos=<0,5,0>, r=3",
            "pos=<0,0,3>, r=1",
            "pos=<1,1,1>, r=1",
            "pos=<1,1,2>, r=1",
            "pos=<1,3,1>, r=1",
        ]
        .as_records::<Nanobot>()
        .unwrap();

        assert_eq!(count_nr_nanobots_in_range_of_strongest(&nanobots), Some(7));
    }

    #[test]
    fn test_find_optimal_position() {
        let nanobots = vec![
            "pos=<10,12,12>, r=2",
            "pos=<12,14,12>, r=2",
            "pos=<16,12,12>, r=4",
            "pos=<14,14,14>, r=6",
            "pos=<50,50,50>, r=200",
            "pos=<10,10,10>, r=5",
        ]
        .as_records::<Nanobot>()
        .unwrap();

        assert_eq!(find_optimal_position(&nanobots), Some((12, 12, 12)));
        assert_eq!(distance_to_origin((12, 12, 12)), 36);
    }
}
