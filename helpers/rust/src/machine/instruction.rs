use std::fmt;
use std::str::FromStr;

use crate::error::ParseError;
use crate::machine::output_receiver::OutputReceiver;
use crate::machine::register::MachineRegister;

/// A machine instruction.
pub trait MachineInstruction: Sized + Clone {
    /// Executes this instruction. Returns an offset indicating the next instruction. For example,
    /// an offset of 1 means "go to the next instruction", an offset of -1 means "go to the previous
    /// instruction", an offset of 3 means "skip two instructions" and an offset of 0 means "repeat
    /// the current instruction". If this offset causes the program counter to point to a
    /// non-existing instruction (i.e. either the counter becomes negative, or greater than the
    /// number of instructions), this will cause the program to halt. If you want to abort the
    /// program, you can just return `i32::MIN`. This will always cause the program counter to
    /// become negative, and will thus halt the program.
    fn execute<R: MachineRegister, O: OutputReceiver<R>>(
        &self,
        register: &mut R,
        output_receiver: &mut O,
    ) -> i32;

    /// Implement this method to be able to use the provided `from_str` method to parse
    /// instructions.
    fn from_parsed_machine_instruction(
        parsed: &ParsedMachineInstruction,
    ) -> Result<Self, ParseError>;

    /// Parses an instruction using `ParsedMachineInstruction` and
    /// `from_parsed_machine_instruction`.
    fn from_str(instruction: &str) -> Result<Self, ParseError> {
        let parsed = instruction.parse::<ParsedMachineInstruction>()?;
        Self::from_parsed_machine_instruction(&parsed)
    }
}

/// A value used within a machine instruction.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Value {
    /// A raw value just contains a number.
    Raw(i32),

    /// A register value refers to the register where the value is stored.
    Register(char),
}

impl Value {
    /// Reads the actual value contained in `self`.
    pub fn get<T: MachineRegister>(&self, registers: &T) -> i32 {
        match self {
            Value::Raw(value) => *value,
            Value::Register(register) => registers.read(*register),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Raw(value) => write!(f, "{}", value),
            Value::Register(register) => write!(f, "{}", register),
        }
    }
}

impl FromStr for Value {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<i32>() {
            Ok(value) => Ok(Value::Raw(value)),
            Err(parse_int_error) => match s.parse::<char>() {
                Ok(register) => Ok(Value::Register(register)),
                Err(parse_char_error) => Err(ParseError(format!(
                    "Failed to parse value. Could not parse as int ({}) or as register ({}).",
                    parse_int_error, parse_char_error,
                ))),
            },
        }
    }
}

/// A parsed machine instruction.
#[derive(Debug)]
pub struct ParsedMachineInstruction {
    command: String,
    arguments: Vec<String>,
}

impl ParsedMachineInstruction {
    #[cfg(test)]
    fn new(command: &str, arguments: &[&str]) -> ParsedMachineInstruction {
        ParsedMachineInstruction {
            command: command.to_string(),
            arguments: arguments.iter().map(|a| a.to_string()).collect(),
        }
    }

    /// Gets the command associated with this instruction.
    pub fn get_command(&self) -> &str {
        self.command.as_str()
    }

    /// Reads the nth argument and parses it to `T`.
    pub fn get_argument<T: FromStr>(&self, idx: usize) -> Result<T, ParseError>
    where
        ParseError: From<<T as FromStr>::Err>,
    {
        match self.arguments.get(idx) {
            Some(value) => Ok(value.parse()?),
            None => Err(ParseError(format!(
                "Command {} has insufficient arguments ({:?})",
                self.command, self.arguments
            ))),
        }
    }
}

impl fmt::Display for ParsedMachineInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.command)?;
        for arg in &self.arguments {
            write!(f, " {}", arg)?;
        }
        Ok(())
    }
}

impl FromStr for ParsedMachineInstruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err(ParseError::of("Could not parse empty instruction"))
        } else {
            let normalized = s.replace(',', " ");
            let mut parts = normalized.split_whitespace();
            let command = match parts.next() {
                Some(cmd) => cmd.to_string(),
                None => unreachable!(),
            };
            let arguments = parts.map(|a| a.to_string()).collect();

            Ok(ParsedMachineInstruction { command, arguments })
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::machine::register::HashMapRegister;

    use super::*;

    #[test]
    fn test_get_raw_value() {
        let value = Value::Raw(10);
        let registers = HashMapRegister::default();
        assert_eq!(value.get(&registers), 10);
    }

    #[test]
    fn test_get_register_value() {
        let value = Value::Register('a');
        let mut registers = HashMapRegister::default();
        registers.write('a', 10);
        assert_eq!(value.get(&registers), 10);
    }

    #[test]
    fn test_format_raw_value() {
        assert_eq!(format!("{}", Value::Raw(10)), "10");
    }

    #[test]
    fn test_format_register_value() {
        assert_eq!(format!("{}", Value::Register('a')), "a");
    }

    #[test]
    fn test_parse_invalid_value() {
        const ERROR_MESSAGE: &str = "Failed to parse value. Could not parse as int (invalid digit found in string) or as register (too many characters in string).";
        assert_eq!("foo".parse::<Value>(), Err(ParseError::of(ERROR_MESSAGE)));
    }

    #[test]
    fn test_parse_raw_value() {
        assert_eq!("+142".parse::<Value>(), Ok(Value::Raw(142)));
    }

    #[test]
    fn test_parse_register_value() {
        assert_eq!("a".parse::<Value>(), Ok(Value::Register('a')));
    }

    #[test]
    fn test_format_instruction() {
        let instruction = ParsedMachineInstruction::new("add", &["10", "15"]);
        assert_eq!(format!("{}", instruction), "add 10 15");
    }

    #[test]
    fn test_parse_empty_instruction() {
        let parsed = "".parse::<ParsedMachineInstruction>().unwrap_err();
        assert_eq!(parsed, ParseError::of("Could not parse empty instruction"));
    }

    #[test]
    fn test_parse_zero_argument_instruction() {
        let parsed = "nop".parse::<ParsedMachineInstruction>().unwrap();
        assert_eq!(parsed.command, "nop");
        assert_eq!(parsed.arguments.len(), 0);
    }

    #[test]
    fn test_parse_instruction_with_arguments() {
        let parsed = "add 5 -10".parse::<ParsedMachineInstruction>().unwrap();
        assert_eq!(parsed.get_command(), "add");
        assert_eq!(parsed.arguments.len(), 2);
        assert_eq!(parsed.get_argument::<i32>(0), Ok(5));
        assert_eq!(parsed.get_argument::<i32>(1), Ok(-10));
    }
}
