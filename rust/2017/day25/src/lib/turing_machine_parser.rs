use std::collections::{HashMap, HashSet};

use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::input::MultilineFromStr;
use rdcl_aoc_helpers::parse_error;

use crate::direction::Direction;
use crate::instruction::Instruction;
use crate::turing_machine::TuringMachine;
use crate::value::Value;

#[derive(Debug, Clone)]
pub struct TuringMachineParser {
    turing_machine: TuringMachine,
    parsing: Parsing,
    parsing_state: char,
    parsing_condition: Value,
    parsing_instruction: Instruction,
}

impl TuringMachineParser {
    pub fn get(&self) -> TuringMachine {
        self.turing_machine.clone()
    }
}

impl MultilineFromStr for TuringMachineParser {
    type Err = ParseError;

    fn new() -> Self {
        TuringMachineParser {
            turing_machine: TuringMachine {
                tape: HashSet::new(),
                cursor: 0,
                states: HashMap::new(),
                diagnostic_checksum_after: 0,
                current_state: '_',
            },
            parsing: Parsing::StartState,
            parsing_state: '_',
            parsing_condition: Value::Zero,
            parsing_instruction: Instruction {
                write: Value::Zero,
                move_cursor: Direction::Right,
                next_state: '_',
            },
        }
    }

    fn indicates_new_record(&self, _line: &str) -> bool {
        false
    }

    fn parse(&mut self, line: &str) -> Result<(), Self::Err> {
        match self.parsing {
            Parsing::StartState => {
                if let Some(r) = line.strip_prefix("Begin in state ") {
                    self.turing_machine.current_state = r[0..1].parse()?;
                } else {
                    return Err(parse_error!(
                        "[{:?}] Invalid input line: {}",
                        self.parsing,
                        line
                    ));
                }
                self.parsing = Parsing::DiagnosticChecksumSteps;
            }
            Parsing::DiagnosticChecksumSteps => {
                let opt = line
                    .strip_prefix("Perform a diagnostic checksum after ")
                    .and_then(|r| r.rfind(' ').map(|idx| (r, idx)));

                if let Some((r, idx)) = opt {
                    self.turing_machine.diagnostic_checksum_after = r[..idx].parse()?;
                } else {
                    return Err(parse_error!(
                        "[{:?}] Invalid input line: {}",
                        self.parsing,
                        line
                    ));
                }
                self.parsing = Parsing::StateHeader;
            }
            Parsing::StateHeader => {
                if !line.is_empty() {
                    if let Some(r) = line.strip_prefix("In state ") {
                        self.parsing_state = r[0..1].parse()?;
                    } else {
                        return Err(parse_error!(
                            "[{:?}] Invalid input line: {}",
                            self.parsing,
                            line
                        ));
                    }
                    self.parsing = Parsing::InstructionCondition;
                }
            }
            Parsing::InstructionCondition => {
                if line.is_empty() {
                    self.parsing = Parsing::StateHeader;
                } else {
                    match line.trim() {
                        "If the current value is 0:" => {
                            self.parsing_condition = Value::Zero;
                        }
                        "If the current value is 1:" => {
                            self.parsing_condition = Value::One;
                        }
                        _ => {
                            return Err(parse_error!(
                                "[{:?}] Invalid input line: {}",
                                self.parsing,
                                line
                            ));
                        }
                    }
                    self.parsing = Parsing::InstructionWrite;
                }
            }
            Parsing::InstructionWrite => {
                match line.trim() {
                    "- Write the value 0." => {
                        self.parsing_instruction.write = Value::Zero;
                    }
                    "- Write the value 1." => {
                        self.parsing_instruction.write = Value::One;
                    }
                    _ => {
                        return Err(parse_error!(
                            "[{:?}] Invalid input line: {}",
                            self.parsing,
                            line
                        ));
                    }
                }
                self.parsing = Parsing::InstructionMove;
            }
            Parsing::InstructionMove => {
                match line.trim() {
                    "- Move one slot to the right." => {
                        self.parsing_instruction.move_cursor = Direction::Right;
                    }
                    "- Move one slot to the left." => {
                        self.parsing_instruction.move_cursor = Direction::Left;
                    }
                    _ => {
                        return Err(parse_error!(
                            "[{:?}] Invalid input line: {}",
                            self.parsing,
                            line
                        ));
                    }
                }
                self.parsing = Parsing::InstructionNextState;
            }
            Parsing::InstructionNextState => {
                if let Some(r) = line.trim().strip_prefix("- Continue with state ") {
                    self.parsing_instruction.next_state = r[0..1].parse()?;
                } else {
                    return Err(parse_error!(
                        "[{:?}] Invalid input line: {}",
                        self.parsing,
                        line
                    ));
                }
                self.turing_machine.states.insert(
                    (self.parsing_state, self.parsing_condition),
                    self.parsing_instruction,
                );
                self.parsing = Parsing::InstructionCondition;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Copy, Clone)]
enum Parsing {
    StartState,
    DiagnosticChecksumSteps,
    StateHeader,
    InstructionCondition,
    InstructionWrite,
    InstructionMove,
    InstructionNextState,
}
