use std::collections::{HashMap, HashSet, VecDeque};
use std::str::FromStr;

use rdcl_aoc_core::error::ParseError;
use rdcl_aoc_core::{err_parse_error, ParseResult};
use rdcl_aoc_math::lcm;

const BROADCASTER: &str = "broadcaster";
const BUTTON: &str = "button";
const SEPARATOR: &str = " -> ";

pub type ModuleMap = HashMap<String, Module>;
pub type ModuleStates = HashMap<String, ModuleState>;

#[derive(Debug, Copy, Clone)]
pub enum Pulse {
    Low,
    High,
}

impl Pulse {
    fn is_high(&self) -> bool {
        matches![self, Pulse::High]
    }

    fn is_low(&self) -> bool {
        matches![self, Pulse::Low]
    }
}

#[derive(Debug, Clone)]
pub enum Module {
    /// There is a single broadcast module (named broadcaster). When it
    /// receives a pulse, it sends the same pulse to all of its
    /// destination modules.
    Broadcaster(Vec<String>),

    /// Flip-flop modules (prefix %) are either on or off; they are
    /// initially off. If a flip-flop module receives a high pulse, it
    /// is ignored and nothing happens. However, if a flip-flop module
    /// receives a low pulse, it flips between on and off. If it was
    /// off, it turns on and sends a high pulse. If it was on, it turns
    /// off and sends a low pulse.
    FlipFlop(String, HashSet<String>, Vec<String>),

    /// Conjunction modules (prefix &) remember the type of the most
    /// recent pulse received from each of their connected input
    /// modules; they initially default to remembering a low pulse for
    /// each input. When a pulse is received, the conjunction module
    /// first updates its memory for that input. Then, if it remembers
    /// high pulses for all inputs, it sends a low pulse; otherwise, it
    /// sends a high pulse.
    Conjunction(String, HashSet<String>, Vec<String>),
}

impl Module {
    pub fn parse_input<T>(input: T) -> ParseResult<ModuleMap>
    where
        T: Iterator<Item = String>,
    {
        let mut parsed = ModuleMap::new();
        let mut mappings: Vec<(String, String)> = vec![];
        for line in input {
            let module = line.parse::<Module>()?;
            for target in module.targets() {
                mappings.push((module.label(), target.clone()));
            }
            parsed.insert(module.label(), module);
        }
        for (from, to) in mappings {
            if let Some(module) = parsed.get_mut(&to) {
                module.register_input(&from);
            }
        }
        Ok(parsed)
    }

    fn is_flip_flop(&self) -> bool {
        matches![self, Module::FlipFlop(_, _, _)]
    }

    fn is_conjunction(&self) -> bool {
        matches![self, Module::Conjunction(_, _, _)]
    }

    fn label(&self) -> String {
        match self {
            Module::Broadcaster(_) => BROADCASTER.to_string(),
            Module::FlipFlop(label, _, _) => label.clone(),
            Module::Conjunction(label, _, _) => label.clone(),
        }
    }

    fn targets(&self) -> &Vec<String> {
        match self {
            Module::Broadcaster(targets) => targets,
            Module::FlipFlop(_, _, targets) => targets,
            Module::Conjunction(_, _, targets) => targets,
        }
    }

    fn register_input(&mut self, label: &str) {
        match self {
            Module::FlipFlop(_, inputs, _) | Module::Conjunction(_, inputs, _) => {
                inputs.insert(label.to_string());
            }
            _ => {}
        }
    }

    fn receive(
        &self,
        input: &str,
        pulse: Pulse,
        states: &mut ModuleStates,
    ) -> Option<(Pulse, Vec<String>)> {
        match self {
            Module::Broadcaster(targets) => Some((pulse, targets.clone())),
            Module::FlipFlop(label, _, targets) => {
                let state = states
                    .get(label)
                    .cloned()
                    .unwrap_or_else(|| self.get_initial_state());

                if pulse.is_low() {
                    let state = state.update(label, pulse);
                    let signal = state.get_signal();
                    states.insert(label.clone(), state);
                    signal.map(|pulse| (pulse, targets.clone()))
                } else {
                    None
                }
            }
            Module::Conjunction(label, _, targets) => {
                let state = states
                    .get(label)
                    .cloned()
                    .unwrap_or_else(|| self.get_initial_state());

                let state = state.update(input, pulse);
                let signal = state.get_signal();
                states.insert(label.clone(), state);
                signal.map(|pulse| (pulse, targets.clone()))
            }
        }
    }

    fn get_initial_state(&self) -> ModuleState {
        match self {
            Module::Broadcaster(_) => unreachable!(),
            Module::FlipFlop(_, _, _) => ModuleState::FlipFlop(false, false),
            Module::Conjunction(_, inputs, _) => {
                let mut states = HashMap::new();
                for input in inputs {
                    states.insert(input.clone(), Pulse::Low);
                }
                ModuleState::Conjunction(states)
            }
        }
    }
}

impl FromStr for Module {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let idx = s.find(SEPARATOR).ok_or(())?;

        let targets = s[idx + SEPARATOR.len()..]
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        let module_str = &s[0..idx];
        if module_str == BROADCASTER {
            Ok(Module::Broadcaster(targets))
        } else if let Some(label) = module_str.strip_prefix('%') {
            Ok(Module::FlipFlop(label.to_string(), HashSet::new(), targets))
        } else if let Some(label) = module_str.strip_prefix('&') {
            Ok(Module::Conjunction(
                label.to_string(),
                HashSet::new(),
                targets,
            ))
        } else {
            err_parse_error!("invalid module: {}", s)
        }
    }
}

/// Used to keep track of the state of a specific module.
#[derive(Debug, Clone)]
pub enum ModuleState {
    /// For flip-flops, the two most recent on/off-states are tracked.
    /// If they differ, then the most recent pulse has caused the
    /// flip-flop to flip.
    FlipFlop(bool, bool),

    /// For conjunctions, the most recent pulse of every single input is
    /// tracked.
    Conjunction(HashMap<String, Pulse>),
}

impl ModuleState {
    fn update(&self, input: &str, pulse: Pulse) -> ModuleState {
        match self {
            ModuleState::FlipFlop(_, state) => match pulse {
                Pulse::Low => ModuleState::FlipFlop(*state, !state),
                Pulse::High => self.clone(),
            },
            ModuleState::Conjunction(states) => {
                let mut next = states.clone();
                next.insert(input.to_string(), pulse);
                ModuleState::Conjunction(next)
            }
        }
    }

    fn get_signal(&self) -> Option<Pulse> {
        match self {
            ModuleState::FlipFlop(false, true) => Some(Pulse::High),
            ModuleState::FlipFlop(true, false) => Some(Pulse::Low),
            ModuleState::FlipFlop(_, _) => None,
            ModuleState::Conjunction(states) => {
                if states.values().all(|v| v.is_high()) {
                    Some(Pulse::Low)
                } else {
                    Some(Pulse::High)
                }
            }
        }
    }

    fn is_on(&self) -> bool {
        match self {
            ModuleState::FlipFlop(_, state) => *state,
            _ => panic!("is_on called on a ModuleState that is not a FlipFlop"),
        }
    }
}

pub trait ButtonModule {
    /// Count the number of low and high pulses that are sent after
    /// pressing the button.
    fn push_button(&self, states: &mut ModuleStates) -> (usize, usize);

    /// Count how many button presses it takes before the given module
    /// is activated by a low pulse.
    fn press_button_until_module_is_activated(&self, target_module: &str) -> usize;
}

impl ButtonModule for ModuleMap {
    fn push_button(&self, states: &mut ModuleStates) -> (usize, usize) {
        let mut signals: VecDeque<(Pulse, String, String)> = VecDeque::new();
        signals.push_back((Pulse::Low, BUTTON.to_string(), BROADCASTER.to_string()));
        let mut pulse_count = (1, 0);

        while let Some((pulse, input, label)) = signals.pop_front() {
            if let Some(module) = self.get(&label) {
                if let Some((pulse, targets)) = module.receive(&input, pulse, states) {
                    for target in targets {
                        if pulse.is_low() {
                            pulse_count.0 += 1;
                        } else {
                            pulse_count.1 += 1;
                        }
                        signals.push_back((pulse, label.clone(), target));
                    }
                }
            }
        }

        pulse_count
    }

    /// Using <https://csacademy.com/app/graph_editor/> to analyze the
    /// input, you will find that the broadcaster sends signals to four
    /// groups of modules. Each group of modules is arranged in a
    /// central conjunction, surrounded by twelve flip-flops. The
    /// broadcaster is connected to a flip-flop, which is connected to
    /// the next flip-flop, and so forth until the twelfth flip-flop.
    /// Each of the flip-flops then either is connected to the central
    /// conjunction, or has the central conjunction connected to it.
    /// Finally, the central conjunction sends its output to an output
    /// conjunction. All four output conjunctions are then connected to
    /// one more final conjunction which then is connected to the output
    /// module.
    ///
    /// For the output to turn on, all of the four central conjunctions
    /// need to be on. This happens whenever when the requisite
    /// flip-flops all send the correct pulse. Each of the groups can be
    /// analysed separately. The final answer can then be deduced from
    /// this.
    fn press_button_until_module_is_activated(&self, _target_module: &str) -> usize {
        let broadcaster = self
            .get(&BROADCASTER.to_string())
            .expect("broadcaster not found");

        let broadcaster_targets = broadcaster.targets();
        let mut groups = broadcaster_targets
            .iter()
            .map(|start| Group::extract(self, start))
            .collect::<Vec<_>>();

        let mut states = ModuleStates::new();
        let mut signals: VecDeque<(Pulse, String, String)> = VecDeque::new();
        let mut count = 0;
        let mut groups_to_update: HashSet<usize> = HashSet::new();
        loop {
            if groups.iter().all(|g| g.is_done()) {
                break;
            }

            count += 1;
            signals.push_back((Pulse::Low, BUTTON.to_string(), BROADCASTER.to_string()));
            while let Some((pulse, input, label)) = signals.pop_front() {
                if let Some(module) = self.get(&label) {
                    if let Some((pulse, targets)) = module.receive(&input, pulse, &mut states) {
                        for (idx, group) in groups.iter_mut().enumerate() {
                            if group.check_pulse(&label, pulse) {
                                group.count_until_first_pulse = count;
                                groups_to_update.insert(idx);
                            }
                        }

                        for target in targets {
                            signals.push_back((pulse, label.clone(), target));
                        }
                    }
                }
            }

            for &idx in &groups_to_update {
                groups[idx].update_flip_flop_states(&states);
            }
            groups_to_update.clear();
        }

        let some_group_has_not_been_reset = groups
            .iter()
            .any(|group| group.values_after_first_pulse != 0);
        if some_group_has_not_been_reset {
            todo!("did not yet implement any logic that deals with the case where the groups do not reset to zero after sending their first pulse")
        }

        groups
            .iter()
            .map(|group| group.count_until_first_pulse)
            .reduce(lcm)
            .unwrap()
    }
}

#[derive(Debug)]
struct Group {
    /// The central conjunction to which all flip flops are connected.
    central_conjunction: String,

    /// The flip flops that connect to the central conjunction.
    flip_flops: Vec<String>,

    /// How often the button needs to be clicked until the central
    /// conjunction sends its first low pulse.
    count_until_first_pulse: usize,

    /// The values of the flip flops after the central conjunction sent
    /// its first pulse. Encoded as an unsigned integer, where flip
    /// flops are encoded as a 1 if they are turned on or as a 0
    /// otherwise. The first flip flop is in the 1's place, the second
    /// flip flop is in the 2's place, etc. For example,
    /// 0b0100_1101_0011 means that the flip flops are, in order: on,
    /// on, off, off, on, off, on, on, off, off, on, off.
    values_after_first_pulse: u16,
}

impl Group {
    fn is_done(&self) -> bool {
        self.count_until_first_pulse != 0
    }

    fn check_pulse(&self, label: &str, pulse: Pulse) -> bool {
        !self.is_done() && self.central_conjunction == label && pulse.is_low()
    }

    fn update_flip_flop_states(&mut self, states: &ModuleStates) {
        for flip_flop in self.flip_flops.iter().rev() {
            let state = states
                .get(flip_flop)
                .expect("module not found in states object");

            self.values_after_first_pulse <<= 1;
            if state.is_on() {
                self.values_after_first_pulse += 1;
            }
        }
    }

    /// Extract a group from the input. A number of assumptions are made
    /// and the extracting will panic if any of these assumptions are
    /// broken.
    fn extract(modules: &ModuleMap, start: &str) -> Group {
        let start_module = modules
            .get(&start.to_string())
            .expect("broadcaster is broadcasting to a non-existent module");
        if !start_module.is_flip_flop() {
            panic!("broadcaster is broadcasting to a module that is not a flip-flop");
        }

        let central_conjunction = start_module
            .targets()
            .iter()
            .map(|target| {
                modules
                    .get(target)
                    .expect("encountered a non-existent module")
            })
            .find(|module| module.is_conjunction())
            .expect("central conjunction not found");

        let mut group = Group {
            central_conjunction: central_conjunction.label(),
            flip_flops: vec![start_module.label()],
            count_until_first_pulse: 0,
            values_after_first_pulse: 0,
        };

        let mut current = start_module.label();
        let mut seen: HashSet<String> = HashSet::new();
        seen.insert(start_module.label());

        loop {
            let module = modules.get(&current).unwrap();
            let next = module
                .targets()
                .iter()
                .map(|target| {
                    modules
                        .get(target)
                        .expect("encountered a non-existent module")
                })
                .find(|module| module.is_flip_flop());

            if let Some(next) = next {
                if seen.contains(&next.label()) {
                    panic!("encountered an unexpected loop within a group");
                }
                group.flip_flops.push(next.label());
                seen.insert(next.label());
                current = next.label();
            } else {
                break;
            }
        }

        group
    }
}
