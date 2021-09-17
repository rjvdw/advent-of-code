extern crate rdcl_aoc_helpers;

use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;
use rdcl_aoc_helpers::input::WithReadLines;

/// https://adventofcode.com/2020/day/10
fn main() {
    let args = get_args(&["<input file>", "<max jolt difference>"], 1);

    let joltage_ratings = File::open(&args[1]).read_lines(1).collect::<Vec<u32>>();
    let max_jolt_difference = args[2].parse::<u32>().or_exit_with(1);

    match calculate_nr_of_min_jolt_differences_times_nr_of_max_jolt_differences(&joltage_ratings, max_jolt_difference) {
        Some(v) => println!("The number of 1-jolt differences multiplied by the number of {}-jolt differences is: {}", max_jolt_difference, v),
        None => println!("Could not create a chain from the outlet to your device."),
    };

    println!(
        "The total number of distinct ways you can arrange the adapters to connect the charging outlet to your device is: {}",
        count_paths_from_outlet_to_device(&joltage_ratings, max_jolt_difference)
    );
}

fn calculate_nr_of_min_jolt_differences_times_nr_of_max_jolt_differences(
    joltage_ratings: &[u32],
    max_jolt_difference: u32,
) -> Option<u32> {
    let mut joltage_ratings = joltage_ratings.to_vec();
    joltage_ratings.sort_unstable();

    let mut diff_lower = 0;
    let mut diff_upper = 1; // always at least 1, because of the final step
    let mut prev = 0;

    for joltage_rating in joltage_ratings {
        if joltage_rating - prev == 1 {
            diff_lower += 1;
        } else if joltage_rating - prev == max_jolt_difference {
            diff_upper += 1;
        } else if joltage_rating - prev > max_jolt_difference {
            return None;
        }

        prev = joltage_rating;
    }

    Some(diff_lower * diff_upper)
}

/// Counts the number of paths from start to end, where each individual step does not exceed a
/// specified threshold.
///
/// We iterate through the list from end to start, and keep track per element how many paths there
/// are from the _current_ element to the end. We can find the next count, by simply checking which
/// elements can be reached from our _current_ element, and then sum the number of paths for those
/// elements. For example, let's say we have the input list [1, 3, 4, 5] and a max jolt
/// difference of 3. We start at 0, and end at the highest element + 3, so let's add those elements
/// to the list to get [0, 1, 3, 4, 5, 8]. We loop through this list in reverse order, so let's
/// start by considering how many paths exist from 5 to 8: Only one single path. Next is 4. The only
/// element that can be reached from 4 is 5, so the number of paths from 4 to 8 is equal to the
/// number of paths from 5 to 8: Only one single path. Next is 3. There are two elements that can be
/// reached from 3: 4 and 5. There is one path from 4 to 8 and one path from 5 to 8, so the number
/// of paths from 3 to 8 is two. We can keep this up all the way to 0, to find that the total number
/// of paths is two.
///
/// One final optimisation: Since the elements in the list are unique, and only have integer values,
/// we only need to keep track of `max_jolt_difference` elements, since each step adds at least one.
fn count_paths_from_outlet_to_device(joltage_ratings: &[u32], max_jolt_difference: u32) -> u64 {
    let mut joltage_ratings = joltage_ratings.to_vec();
    joltage_ratings.push(0); // we start at 0
    joltage_ratings.sort_unstable();
    joltage_ratings.push(joltage_ratings[joltage_ratings.len() - 1] + max_jolt_difference); // and end at the target
    let joltage_ratings = joltage_ratings;

    let result_size = max_jolt_difference as usize;
    let mut result: Vec<u64> = vec![0; result_size];
    result[(joltage_ratings.len() - 1) % result_size] = 1;

    for (idx, joltage_rating) in joltage_ratings.iter().enumerate().rev().skip(1) {
        result[idx % result_size] = joltage_ratings
            .iter()
            .skip(idx + 1)
            .take_while(|&&next_joltage_rating| {
                next_joltage_rating - joltage_rating <= max_jolt_difference
            })
            .enumerate()
            .map(|(pos, _)| result[(pos + idx + 1) % result_size])
            .sum();
    }

    result[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    mod test_calculate_nr_of_min_jolt_differences_times_nr_of_max_jolt_differences {
        use super::*;

        #[test]
        fn test_1() {
            let joltage_ratings = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];

            assert_eq!(
                calculate_nr_of_min_jolt_differences_times_nr_of_max_jolt_differences(
                    &joltage_ratings,
                    3,
                ),
                Some(35)
            );
        }

        #[test]
        fn test_2() {
            let joltage_ratings = vec![
                28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25,
                35, 8, 17, 7, 9, 4, 2, 34, 10, 3,
            ];

            assert_eq!(
                calculate_nr_of_min_jolt_differences_times_nr_of_max_jolt_differences(
                    &joltage_ratings,
                    3,
                ),
                Some(220)
            );
        }

        #[test]
        fn test_3() {
            let joltage_ratings = vec![];

            assert_eq!(
                calculate_nr_of_min_jolt_differences_times_nr_of_max_jolt_differences(
                    &joltage_ratings,
                    3,
                ),
                Some(0)
            );
        }

        #[test]
        fn test_4() {
            let joltage_ratings = vec![3, 9];

            assert_eq!(
                calculate_nr_of_min_jolt_differences_times_nr_of_max_jolt_differences(
                    &joltage_ratings,
                    3,
                ),
                None
            );
        }
    }

    mod test_count_paths_from_outlet_to_device {
        use super::*;

        #[test]
        fn test_1() {
            let joltage_ratings = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];

            assert_eq!(count_paths_from_outlet_to_device(&joltage_ratings, 3), 8);
        }

        #[test]
        fn test_2() {
            let joltage_ratings = vec![
                28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25,
                35, 8, 17, 7, 9, 4, 2, 34, 10, 3,
            ];

            assert_eq!(
                count_paths_from_outlet_to_device(&joltage_ratings, 3),
                19208
            );
        }
    }
}
