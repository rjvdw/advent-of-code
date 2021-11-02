use std::collections::HashMap;

const MINUTES: u32 = 60;

pub struct SleepAnalysis {
    slept_for: HashMap<usize, u32>,
    frequencies: HashMap<usize, [u32; MINUTES as usize]>,
    current_guard: usize,
    asleep_since: u32,
    awake: bool,
}

impl SleepAnalysis {
    pub fn get_biggest_sleeper(&self) -> Option<(usize, u32, usize)> {
        self.slept_for
            .iter()
            .max_by_key(|(_, &s)| s)
            .map(|(guard, slept)| {
                (
                    *guard,
                    *slept,
                    self.get_most_frequent_sleeping_minute(*guard),
                )
            })
    }

    pub fn get_most_reliable_sleeper(&self) -> Option<(usize, u32, usize)> {
        self.frequencies
            .iter()
            .map(|(&guard, f)| f.iter().enumerate().map(move |(idx, v)| (guard, idx, *v)))
            .flatten()
            .max_by_key(|(_, _, v)| *v)
            .map(|(guard, minute, _)| (guard, *self.slept_for.get(&guard).unwrap_or(&0), minute))
    }

    pub fn get_most_frequent_sleeping_minute(&self, guard: usize) -> usize {
        self.frequencies
            .get(&guard)
            .map(|f| f.iter().enumerate().max_by_key(|(_, &v)| v))
            .flatten()
            .map(|(idx, _)| idx)
            .unwrap_or(0)
    }

    pub fn start_shift(&mut self, next_guard: usize) {
        if !self.awake {
            self.log_sleep_time(MINUTES);
        }
        self.current_guard = next_guard;
    }

    pub fn log_sleep_time(&mut self, now: u32) {
        *self.slept_for.entry(self.current_guard).or_insert(0) += now - self.asleep_since;
        self.awake = true;

        for minute in self.asleep_since..now {
            self.frequencies
                .entry(self.current_guard)
                .or_insert([0; MINUTES as usize])[minute as usize] += 1;
        }
    }

    pub fn fall_asleep(&mut self, now: u32) {
        self.asleep_since = now;
        self.awake = false;
    }
}

impl Default for SleepAnalysis {
    fn default() -> Self {
        SleepAnalysis {
            slept_for: HashMap::new(),
            frequencies: HashMap::new(),
            current_guard: 0,
            asleep_since: 0,
            awake: true,
        }
    }
}
