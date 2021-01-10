use rdcl_aoc_helpers::machine::hook::{HookResult, PreExecuteHook};
use rdcl_aoc_helpers::machine::output_receiver::OutputReceiver;
use rdcl_aoc_helpers::machine::register::MachineRegister;
use rdcl_aoc_helpers::machine::Machine;

use crate::instruction::Instruction;

pub struct AbortingHook {
    first_recovered_sound: Option<i64>,
    last_sound: Option<i64>,
}

impl AbortingHook {
    pub fn new() -> AbortingHook {
        AbortingHook {
            first_recovered_sound: None,
            last_sound: None,
        }
    }

    fn play(&mut self, sound: i64) {
        if sound == 0 {
            self.last_sound = None;
        } else {
            self.last_sound = Some(sound);
        }
    }

    fn recover(&mut self) -> Option<i64> {
        if self.first_recovered_sound.is_none() {
            self.first_recovered_sound = self.last_sound;
        }
        self.last_sound
    }

    pub fn get_first_recovered_sound(&self) -> Option<i64> {
        self.first_recovered_sound
    }
}

impl PreExecuteHook<Instruction> for AbortingHook {
    fn run<R: MachineRegister, O: OutputReceiver<R>>(
        &mut self,
        machine: &mut Machine<Instruction, R, O>,
        instruction: &Instruction,
        _idx: usize,
    ) -> HookResult {
        match instruction {
            Instruction::Sound(a) => {
                self.play(a.get(&machine.register));
                HookResult::Skip
            }
            Instruction::Recover(a) => {
                if let Some(sound) = self.recover() {
                    machine.register.write(*a, sound);
                    HookResult::Abort
                } else {
                    HookResult::Skip
                }
            }
            _ => HookResult::Proceed,
        }
    }
}
