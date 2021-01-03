use crate::machine::register::MachineRegister;

/// If your machine produces output, it should send this to an OutputReceiver.
pub trait OutputReceiver {
    /// Receive some output. Returns a boolean (up to the implementer on how to interpret this).
    fn receive<T: MachineRegister>(&mut self, output: i32, registers: &T) -> bool;
}

/// If your machine does not produce output, you can use this implementation.
pub struct NoopOutputReceiver;

impl OutputReceiver for NoopOutputReceiver {
    fn receive<T: MachineRegister>(&mut self, _output: i32, _registers: &T) -> bool {
        false
    }
}
