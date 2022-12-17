use std::collections::{HashMap, HashSet};

use rdcl_aoc_core::err_parse_error;
use rdcl_aoc_core::error::ParseError;
use rdcl_aoc_pathfinding::AStar;

use crate::label::Label;
use crate::tunnels::Tunnels;
use crate::valve::Valve;

#[derive(Debug, Default)]
pub struct Volcano {
    valves: HashMap<Label, Valve>,
    useful_valves: HashMap<Label, Valve>,
    connections: HashMap<(Label, Label), usize>,
}

impl Volcano {
    pub fn find_max_pressure_relief(&self, start: Label, time_remaining: usize) -> usize {
        let mut best = 0;
        self.find(
            (start, time_remaining),
            (start, 0),
            &mut best,
            self.useful_valves.len(),
            0,
            &mut HashSet::new(),
        );
        best
    }

    pub fn find_max_pressure_relief_with_elephant(
        &self,
        start: Label,
        time_remaining: usize,
    ) -> usize {
        let mut best = 0;
        for division in (0..=self.useful_valves.len() / 2).rev() {
            let result = self.find(
                (start, time_remaining),
                (start, time_remaining),
                &mut best,
                division,
                0,
                &mut HashSet::new(),
            );
            println!("{} -- {}", division, result);
        }
        best
    }

    /// Work together with an elephant to find the optimal strategy.
    ///
    /// You will start picking which valves you are going to open, and the elephant will open the remaining valves.
    /// The parameter `division` indicates how the work is divided.
    /// This number indicates how many valves should be opened by you.
    ///
    /// * `you` - Your current position and how much time there is left.
    /// * `elephant` - The current position of the elephant and how much time there is left.
    /// * `division` - How many valves should `you` open?
    /// * `relieved` - How much pressure has been relieved so far.
    /// * `valves` - The valves which are relevant (i.e. do not have a flow rate of 0).
    /// * `open` - Which valves are currently open.
    fn find(
        &self,
        you: (Label, usize),
        elephant: (Label, usize),
        best_so_far: &mut usize,
        division: usize,
        relieved: usize,
        open: &mut HashSet<Label>,
    ) -> usize {
        let (start, time_remaining) = if division > 0 { you } else { elephant };

        // Start by applying a heuristic. Compute an upper bound for the best possible result from
        // the current situation. If this upper bound is lower than the best score that was found so
        // far, then there is no point continuing down this branch.
        let upper_bound = self.compute_upper_bound_for_relief(you, elephant, open, division);
        if relieved + upper_bound < *best_so_far {
            // no point going any further
            return 0;
        }

        // Determine which valves can still be opened, given the remaining time.
        let mut valves: Vec<(Label, Valve, usize)> = self
            .useful_valves
            .iter()
            .filter(|(label, _)| !open.contains(label))
            .map(|(&label, &valve)| {
                (
                    label,
                    valve,
                    *self.connections.get(&(start, label)).unwrap(),
                )
            })
            .filter(|(_, _, distance)| *distance < time_remaining)
            .collect();

        // Sort the valves by the highest possible relief they can provide. Try the valves with the
        // highest relief first.
        valves.sort_unstable_by_key(|(_, valve, distance)| {
            valve.flow_rate() * (time_remaining - *distance)
        });

        let mut best = relieved;
        for (label, valve, distance) in valves.into_iter().rev() {
            open.insert(label);
            let time_remaining = time_remaining - distance;
            let relieved = relieved + time_remaining * valve.flow_rate();

            best = best.max(if division > 0 {
                self.find(
                    (label, time_remaining),
                    elephant,
                    best_so_far,
                    division - 1,
                    relieved,
                    open,
                )
            } else {
                self.find(
                    you,
                    (label, time_remaining),
                    best_so_far,
                    division,
                    relieved,
                    open,
                )
            });
            *best_so_far = (*best_so_far).max(best);

            open.remove(&label);
        }
        best
    }

    /// Upper bound for the amount of pressure that could possibly be relieved.
    fn compute_upper_bound_for_relief(
        &self,
        you: (Label, usize),
        elephant: (Label, usize),
        open: &HashSet<Label>,
        division: usize,
    ) -> usize {
        let mut score = 0;

        for (&label, &valve) in &self.useful_valves {
            if !open.contains(&label) {
                let score_you = if division > 0 {
                    let (position, time_remaining) = you;
                    let distance = *self.connections.get(&(position, label)).unwrap();
                    if distance < time_remaining {
                        valve.flow_rate() * (time_remaining - distance)
                    } else {
                        0
                    }
                } else {
                    0
                };

                let (position, time_remaining) = elephant;
                let distance = *self.connections.get(&(position, label)).unwrap();
                let score_elephant = if distance < time_remaining {
                    valve.flow_rate() * (time_remaining - distance)
                } else {
                    0
                };

                score += score_you.max(score_elephant);
            }
        }

        score
    }

    pub fn parse<T>(input: T) -> Result<Volcano, ParseError>
    where
        T: Iterator<Item = String>,
    {
        let mut volcano = Volcano::default();
        let mut labels: Vec<Label> = vec![];
        let mut tunnels: Tunnels = Tunnels::default();

        for line in input {
            let i = match line.find("; ") {
                Some(i) => i,
                None => {
                    return err_parse_error!("Invalid input: {}", line);
                }
            };

            let valve = line[..i].parse::<Valve>()?;
            labels.push(valve.label());
            volcano.valves.insert(valve.label(), valve);
            if valve.flow_rate() > 0 {
                volcano.useful_valves.insert(valve.label(), valve);
            }

            let sub = match line[i + 2..].strip_prefix("tunnels lead to valves ") {
                Some(s) => s,
                None => match line[i + 2..].strip_prefix("tunnel leads to valve ") {
                    Some(s) => s,
                    None => {
                        return err_parse_error!("Invalid input: {}", line);
                    }
                },
            };

            let mut targets: HashSet<Label> = HashSet::new();
            for label in sub.split(", ") {
                targets.insert(label[..2].parse::<Label>()?);
            }
            tunnels.insert(valve.label(), targets);
        }

        for (i, label1) in labels.iter().enumerate() {
            for label2 in labels.iter().skip(i + 1) {
                let distance = tunnels.find_shortest_path(label1, label2).unwrap();
                volcano
                    .connections
                    .insert((*label1, *label2), distance.len());
                volcano
                    .connections
                    .insert((*label2, *label1), distance.len());
            }
        }

        Ok(volcano)
    }
}
