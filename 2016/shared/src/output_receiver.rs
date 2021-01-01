use std::collections::HashMap;

pub trait OutputReceiver {
    /// Receives output, and returns whether the program should abort.
    fn receive(&mut self, output: i32, registers: &HashMap<char, i32>) -> bool;
}

pub struct NoopOutputReceiver;

impl OutputReceiver for NoopOutputReceiver {
    fn receive(&mut self, _: i32, _: &HashMap<char, i32>) -> bool {
        false
    }
}
