use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::str::FromStr;

use rdcl_aoc2023::str_encoder::{decode_str, encode_str};
use rdcl_aoc_core::error::ParseError;
use rdcl_aoc_core::{err_parse_error, ParseResult};

const BROADCASTER_LABEL: u32 = 0;
const BROADCASTER_STR: &str = "broadcaster";
const BUTTON_LABEL: u32 = 1;
pub const SEPARATOR: &str = " -> ";

pub type ModuleMap = HashMap<u32, Module>;
pub type ModuleStates = HashMap<u32, ModuleState>;

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
    Broadcaster(Vec<u32>),
    FlipFlop(u32, Vec<u32>),
    Conjunction(u32, HashSet<u32>, Vec<u32>),
}

impl Module {
    pub fn parse_input<T>(input: T) -> ParseResult<ModuleMap>
    where
        T: Iterator<Item = String>,
    {
        let mut parsed = ModuleMap::new();
        let mut mappings: Vec<(u32, u32)> = vec![];
        for line in input {
            let module = line.parse::<Module>()?;
            for target in &module.targets() {
                mappings.push((module.label(), *target));
            }
            parsed.insert(module.label(), module);
        }
        for (from, to) in mappings {
            if let Some(module) = parsed.get_mut(&to) {
                module.register_input(from);
            }
        }
        Ok(parsed)
    }

    fn label(&self) -> u32 {
        match self {
            Module::Broadcaster(_) => BROADCASTER_LABEL,
            Module::FlipFlop(label, _) => *label,
            Module::Conjunction(label, _, _) => *label,
        }
    }

    fn targets(&self) -> Vec<u32> {
        match self {
            Module::Broadcaster(target) => target.clone(),
            Module::FlipFlop(_, target) => target.clone(),
            Module::Conjunction(_, _, target) => target.clone(),
        }
    }

    fn register_input(&mut self, label: u32) {
        if let Module::Conjunction(_, inputs, _) = self {
            inputs.insert(label);
        }
    }

    fn receive(
        &self,
        input: u32,
        pulse: Pulse,
        states: &mut ModuleStates,
    ) -> Option<(Pulse, Vec<u32>)> {
        match self {
            Module::Broadcaster(targets) => Some((pulse, targets.clone())),
            Module::FlipFlop(label, targets) => {
                let state = states
                    .get(label)
                    .cloned()
                    .unwrap_or_else(|| self.get_default_state());

                if pulse.is_low() {
                    let state = state.update(*label, pulse);
                    let signal = state.get_signal();
                    states.insert(*label, state);
                    signal.map(|pulse| (pulse, targets.clone()))
                } else {
                    None
                }
            }
            Module::Conjunction(label, _, targets) => {
                let state = states
                    .get(label)
                    .cloned()
                    .unwrap_or_else(|| self.get_default_state());

                let state = state.update(input, pulse);
                let signal = state.get_signal();
                states.insert(*label, state);
                signal.map(|pulse| (pulse, targets.clone()))
            }
        }
    }

    fn get_default_state(&self) -> ModuleState {
        match self {
            Module::Broadcaster(_) => unreachable!(),
            Module::FlipFlop(_, _) => ModuleState::FlipFlop(false, false),
            Module::Conjunction(_, inputs, _) => {
                let mut states = HashMap::new();
                for input in inputs {
                    states.insert(*input, Pulse::Low);
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
            .map(encode_str)
            .collect::<Vec<_>>();

        let module_str = &s[0..idx];
        if module_str == BROADCASTER_STR {
            Ok(Module::Broadcaster(targets))
        } else if let Some(label) = module_str.strip_prefix('%') {
            Ok(Module::FlipFlop(encode_str(label), targets))
        } else if let Some(label) = module_str.strip_prefix('&') {
            Ok(Module::Conjunction(
                encode_str(label),
                HashSet::new(),
                targets,
            ))
        } else {
            err_parse_error!("invalid module: {}", s)
        }
    }
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let targets = match self {
            Module::Broadcaster(targets) => {
                write!(f, "broadcaster")?;
                targets
            }
            Module::FlipFlop(label, targets) => {
                write!(f, "%{}", decode_str(*label))?;
                targets
            }
            Module::Conjunction(label, _, targets) => {
                write!(f, "&{}", decode_str(*label))?;
                targets
            }
        };
        write!(f, " ->")?;
        for target in targets {
            write!(f, " {}", decode_str(*target))?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum ModuleState {
    FlipFlop(bool, bool),
    Conjunction(HashMap<u32, Pulse>),
}

impl ModuleState {
    fn update(&self, input: u32, pulse: Pulse) -> ModuleState {
        match self {
            ModuleState::FlipFlop(_, state) => match pulse {
                Pulse::Low => ModuleState::FlipFlop(*state, !state),
                Pulse::High => self.clone(),
            },
            ModuleState::Conjunction(states) => {
                let mut next = states.clone();
                next.insert(input, pulse);
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
}

pub trait ButtonModule {
    fn push_button(&self, states: &mut ModuleStates) -> (usize, usize);
    fn push_button_with_target(&self, states: &mut ModuleStates, sand_module: &str) -> bool;
}

impl ButtonModule for ModuleMap {
    fn push_button(&self, states: &mut ModuleStates) -> (usize, usize) {
        let mut signals: VecDeque<(Pulse, u32, u32)> = VecDeque::new();
        signals.push_back((Pulse::Low, BUTTON_LABEL, BROADCASTER_LABEL));
        let mut pulse_count = (1, 0);

        while let Some((pulse, input, label)) = signals.pop_front() {
            if let Some(module) = self.get(&label) {
                if let Some((pulse, targets)) = module.receive(input, pulse, states) {
                    for target in targets {
                        if pulse.is_low() {
                            pulse_count.0 += 1;
                        } else {
                            pulse_count.1 += 1;
                        }
                        signals.push_back((pulse, label, target));
                    }
                }
            }
        }

        pulse_count
    }

    fn push_button_with_target(&self, states: &mut ModuleStates, sand_module: &str) -> bool {
        let sand_module = encode_str(sand_module);
        let mut signals: VecDeque<(Pulse, u32, u32)> = VecDeque::new();
        signals.push_back((Pulse::Low, BUTTON_LABEL, BROADCASTER_LABEL));

        while let Some((pulse, input, label)) = signals.pop_front() {
            if let Some(module) = self.get(&label) {
                if let Some((pulse, targets)) = module.receive(input, pulse, states) {
                    for target in targets {
                        if target == sand_module && pulse.is_low() {
                            return true;
                        }
                        signals.push_back((pulse, label, target));
                    }
                }
            }
        }

        false
    }
}
