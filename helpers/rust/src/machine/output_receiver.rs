use crate::machine::register::MachineRegister;
use std::fmt::Debug;

/// If your machine produces output, it should send this to an OutputReceiver.
pub trait OutputReceiver<T: MachineRegister>: Debug {
    /// Receive some output. Returns a boolean (up to the implementer on how to interpret this).
    fn receive(&mut self, output: i64, register: &T) -> bool;
}

/// If your machine does not produce output, you can use this implementation.
#[derive(Debug)]
pub struct NoopOutputReceiver;

impl<T: MachineRegister> OutputReceiver<T> for NoopOutputReceiver {
    fn receive(&mut self, _output: i64, _register: &T) -> bool {
        false
    }
}
