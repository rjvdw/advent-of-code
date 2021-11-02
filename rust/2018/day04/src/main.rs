use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::input::WithReadLines;

use crate::log_entry::LogEntry;
use crate::sleep_analysis::SleepAnalysis;

mod log_entry;
mod sleep_analysis;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let mut log_entries = File::open(&args[1])
        .read_lines(1)
        .collect::<Vec<LogEntry>>();
    log_entries.sort_unstable_by_key(|e| e.get_timestamp());

    let analysis = perform_sleep_analysis(&log_entries);
    match analysis.get_biggest_sleeper() {
        Some((guard, minutes, most_frequent_sleeping_minute)) => println!(
            "Guard {} has slept the most, for a total of {} minutes. The guard was most frequently asleep at minute {}. This means the answer to part 1 is {}.",
            guard, minutes, most_frequent_sleeping_minute, guard * most_frequent_sleeping_minute
        ),
        None => eprintln!("No guard ever slept."),
    }
    match analysis.get_most_reliable_sleeper() {
        Some((guard, minutes, most_frequent_sleeping_minute)) => println!(
            "Guard {} has slept the most reliably, for a total of {} minutes. The guard was most frequently asleep at minute {}. This means the answer to part 2 is {}.",
            guard, minutes, most_frequent_sleeping_minute, guard * most_frequent_sleeping_minute
        ),
        None => eprintln!("No guard ever slept."),
    }
}

fn perform_sleep_analysis(log_entries: &[LogEntry]) -> SleepAnalysis {
    let mut analysis = SleepAnalysis::default();

    for entry in log_entries {
        match entry {
            LogEntry::StartShift(_, guard) => {
                analysis.start_shift(*guard);
            }
            LogEntry::FallAsleep(timestamp) => {
                analysis.fall_asleep(timestamp.get_minute());
            }
            LogEntry::WakesUp(timestamp) => {
                analysis.log_sleep_time(timestamp.get_minute());
            }
        }
    }

    analysis
}

#[cfg(test)]
mod tests {
    use rdcl_aoc_helpers::input::WithAsRecords;

    use super::*;

    #[test]
    fn test_get_biggest_sleeper() {
        let log_entries = vec![
            "[1518-11-01 00:00] Guard #10 begins shift",
            "[1518-11-01 00:05] falls asleep",
            "[1518-11-01 00:25] wakes up",
            "[1518-11-01 00:30] falls asleep",
            "[1518-11-01 00:55] wakes up",
            "[1518-11-01 23:58] Guard #99 begins shift",
            "[1518-11-02 00:40] falls asleep",
            "[1518-11-02 00:50] wakes up",
            "[1518-11-03 00:05] Guard #10 begins shift",
            "[1518-11-03 00:24] falls asleep",
            "[1518-11-03 00:29] wakes up",
            "[1518-11-04 00:02] Guard #99 begins shift",
            "[1518-11-04 00:36] falls asleep",
            "[1518-11-04 00:46] wakes up",
            "[1518-11-05 00:03] Guard #99 begins shift",
            "[1518-11-05 00:45] falls asleep",
            "[1518-11-05 00:55] wakes up",
        ]
        .as_records::<LogEntry>()
        .unwrap();

        let answer = perform_sleep_analysis(&log_entries).get_biggest_sleeper();

        assert_eq!(answer, Some((10, 50, 24)));
    }

    #[test]
    fn test_get_most_reliable_sleeper() {
        let log_entries = vec![
            "[1518-11-01 00:00] Guard #10 begins shift",
            "[1518-11-01 00:05] falls asleep",
            "[1518-11-01 00:25] wakes up",
            "[1518-11-01 00:30] falls asleep",
            "[1518-11-01 00:55] wakes up",
            "[1518-11-01 23:58] Guard #99 begins shift",
            "[1518-11-02 00:40] falls asleep",
            "[1518-11-02 00:50] wakes up",
            "[1518-11-03 00:05] Guard #10 begins shift",
            "[1518-11-03 00:24] falls asleep",
            "[1518-11-03 00:29] wakes up",
            "[1518-11-04 00:02] Guard #99 begins shift",
            "[1518-11-04 00:36] falls asleep",
            "[1518-11-04 00:46] wakes up",
            "[1518-11-05 00:03] Guard #99 begins shift",
            "[1518-11-05 00:45] falls asleep",
            "[1518-11-05 00:55] wakes up",
        ]
        .as_records::<LogEntry>()
        .unwrap();

        let answer = perform_sleep_analysis(&log_entries).get_most_reliable_sleeper();

        assert_eq!(answer, Some((99, 30, 45)));
    }
}
