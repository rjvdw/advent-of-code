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
    connections: HashMap<(Label, Label), usize>,
}

impl Volcano {
    fn useful_valves(&self) -> impl Iterator<Item = Label> + '_ {
        self.valves
            .iter()
            .filter(|(_, valve)| valve.flow_rate() > 0)
            .map(|(label, _)| *label)
    }

    pub fn find_max_pressure_relief(&self, start: Label, time_limit: usize) -> usize {
        let valves: HashSet<Label> = self.useful_valves().collect();

        self.find(
            (start, time_limit),
            (start, 0),
            valves.len(),
            0,
            &valves,
            &mut HashSet::new(),
        )
    }

    pub fn find_max_pressure_relief_with_elephant(&self, start: Label, time_limit: usize) -> usize {
        let valves: HashSet<Label> = self.useful_valves().collect();

        let mut best = 0;
        for division in 0..=valves.len() / 2 {
            let result = self.find(
                (start, time_limit),
                (start, time_limit),
                division,
                0,
                &valves,
                &mut HashSet::new(),
            );
            println!("{} -- {}", division, result);
            best = best.max(result);
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
        division: usize,
        relieved: usize,
        valves: &HashSet<Label>,
        open: &mut HashSet<Label>,
    ) -> usize {
        let mut best = relieved;

        for valve in valves {
            if open.contains(valve) {
                continue;
            }

            if division > 0 {
                let (start, time_limit) = you;
                let distance = *self.connections.get(&(start, *valve)).unwrap();
                if distance > time_limit {
                    continue;
                }

                open.insert(*valve);
                let time_limit = time_limit - distance;
                let flow_rate = self.valves.get(valve).unwrap().flow_rate() as usize;
                let relieved = relieved + time_limit * flow_rate;
                best = best.max(self.find(
                    (*valve, time_limit),
                    elephant,
                    division - 1,
                    relieved,
                    valves,
                    open,
                ));
                open.remove(valve);
            } else {
                let (start, time_limit) = elephant;
                let distance = *self.connections.get(&(start, *valve)).unwrap();
                if distance > time_limit {
                    continue;
                }

                open.insert(*valve);
                let time_limit = time_limit - distance;
                let flow_rate = self.valves.get(valve).unwrap().flow_rate() as usize;
                let relieved = relieved + time_limit * flow_rate;
                best = best.max(self.find(
                    you,
                    (*valve, time_limit),
                    division,
                    relieved,
                    valves,
                    open,
                ));
                open.remove(valve);
            }
        }

        best
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
