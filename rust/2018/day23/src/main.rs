use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::input::WithReadLines;
use rdcl_aoc_helpers::math::taxi_cab_3d;

use crate::nanobot::Nanobot;
use crate::region::Region;

mod nanobot;
mod region;
#[macro_use]
mod edge_checks;

/// A point represents a coordinate within 3D space.
pub type Point = (i64, i64, i64);

/// The origin is at 0,0,0.
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
                "You should position yourself at {:?}. This point has distance {} from {:?}.",
                position,
                distance_to_origin(position),
                ORIGIN
            )
        }
        None => eprintln!("Empty list provided."),
    }
}

/// Find the strongest nanobot (i.e. the nanobot with the largest sphere of influence), and count
/// how many nanobots are within the sphere of influence of this nanobot.
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

/// Find the optimal position. A position is optimal if it is within the spheres of influence of the
/// maximum number of nanobots. If there are multiple such points, then the optimal points should be
/// closest to the origin. If multiple points are at the same minimum distance from the origin,
/// while they all are in the maximum number of spheres of influence, then just pick any of these
/// points.
fn find_optimal_position(nanobots: &[Nanobot]) -> Option<Point> {
    // Start by determining the initial bounds to the region. Take the smallest boundaries such that
    // all nanobots fit within this initial region.
    let mut lower_bound = (i64::MAX, i64::MAX, i64::MAX);
    let mut upper_bound = (i64::MIN, i64::MIN, i64::MIN);

    for nanobot in nanobots {
        update_bounds(&mut lower_bound.0, &mut upper_bound.0, nanobot.position.0);
        update_bounds(&mut lower_bound.1, &mut upper_bound.1, nanobot.position.1);
        update_bounds(&mut lower_bound.2, &mut upper_bound.2, nanobot.position.2);
    }

    // This variable will hold the best point we have encountered until now. At the end of this
    // method, it should contain the optimal point. The number stored with this point is the number
    // of nanobots for which this point falls within their sphere of influence.
    let mut best_so_far: Option<(usize, Point)> = None;

    // Regions we are currently exploring. This queue will always be sorted such that the regions
    // with the highest scores are at the start. The score of a region is the number of nanobots for
    // which their sphere of influence overlaps with this region. This score basically gives an
    // upper bound for the best possible point we can find -- no point within this region can ever
    // be in the sphere of influence of more nanobots than this score.
    let mut regions: VecDeque<(usize, Region)> = VecDeque::new();
    regions.push_back((
        nanobots.len(),
        Region {
            from: lower_bound,
            to: upper_bound,
        },
    ));

    // As long as there are still regions left to explore, we take the first one (i.e. the one with
    // the best score).
    while let Some((_, region)) = regions.pop_front() {
        // Check if the region is small enough that we can just loop over all points contained
        // within this region.
        if region.is_small_enough() {
            for point in region.iter() {
                let nanobot_count = nanobots.iter().filter(|b| b.in_range(point)).count();

                // If we have found a better result than what we have found so far, we update the
                // value in `best_so_far`, and we remove all regions that have now become
                // unacceptable given this new count.
                if is_better_result(&best_so_far, (nanobot_count, point)) {
                    best_so_far = Some((nanobot_count, point));
                    while let Some(el) = regions.pop_back() {
                        if is_acceptable_score(el.0, &best_so_far) {
                            regions.push_back(el);
                            break;
                        }
                    }
                }
            }
        } else {
            splice_subregions(nanobots, &region, &mut regions, &best_so_far);
        }
    }

    best_so_far.map(|(_, p)| p)
}

/// Split a region into sub regions, and insert these sub regions into the correct place within the
/// queue. Any sub region that has a score that is not acceptable will be dropped.
fn splice_subregions(
    nanobots: &[Nanobot],
    region: &Region,
    regions: &mut VecDeque<(usize, Region)>,
    best_so_far: &Option<(usize, Point)>,
) {
    // We start by making a vector and moving all regions into this vector. This is so that we can
    // sort this vector after adding the newly found regions.
    // TODO: It might be more efficient to insert using a binary search.
    let mut rc = Vec::with_capacity(regions.len());
    while let Some(r) = regions.pop_back() {
        rc.push(r);
    }

    // Split the region, filter out any unacceptable regions, and add them to the vector.
    region
        .split()
        .iter()
        .map(|&sub_region| {
            let score = sub_region.count_influencing_nanobots(nanobots);
            (score, sub_region)
        })
        .filter(|&(score, _)| is_acceptable_score(score, best_so_far))
        .for_each(|el| rc.push(el));

    // Sort the vector and move the elements back into the queue. Note that we sort the vector from
    // small to large, and then move the elements into the queue in the reverse order.
    rc.sort_unstable_by_key(|(s, _)| *s);
    while let Some((score, region)) = rc.pop() {
        regions.push_back((score, region));
    }
}

/// Returns the distance between this point and the origin.
pub fn distance_to_origin(point: Point) -> i64 {
    taxi_cab_3d(point, ORIGIN)
}

/// Compares two results and returns whether the newest result is better than the best so far.
fn is_better_result(best_so_far: &Option<(usize, Point)>, next: (usize, Point)) -> bool {
    match best_so_far {
        Some(current) => match current.0.cmp(&next.0) {
            Ordering::Less => true,
            Ordering::Equal => distance_to_origin(next.1) < distance_to_origin(current.1),
            Ordering::Greater => false,
        },
        None => true,
    }
}

/// A score is considered acceptable if it is greater than zero, and if it's not worse than the best
/// score we have found so far. It is acceptable if the score is _equal_ to the best score we have
/// found so far.
fn is_acceptable_score(score: usize, best_so_far: &Option<(usize, Point)>) -> bool {
    score != 0 && (best_so_far.is_none() || score >= best_so_far.unwrap().0)
}

/// Given a value, update the min and max bound so that this value fits between them.
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
