# Helpers for [Advent of Code][aoc]

This crate contains some helper methods that I regularly use in my [Advent of Code][aoc] solutions.

## Processing of command line arguments

### `args::get_args`

Reads the command line arguments and checks whether the correct number of arguments are present.

#### Example

```rust
use rdcl_aoc_helpers::args::get_args;

fn example1() {
    let args = get_args(&["<input file>", "<init>"], 1);
    println!("{:?}", args);
}

fn example2() {
    let args = get_args_repeating(&["<input file>", "<x1> ... <xn>"], 1);
    println!("{:?}", args);
}
```

## Error handling

### `error::WithOrExit`

This trait adds a `or_exit_with` method.
The purpose of this method, is to allow you to easily let your program terminate with a specific exit code.
It has been implemented for [Result] and [Option].
The implementation for [Result] requires that the associated error type implements [fmt::Debug].

#### Example

```rust
use rdcl_aoc_helpers::error::WithOrExit;

fn main() {
    some_operation_that_returns_a_result()
        .or_exit_with(25);
}
```

### `error::ParseError`

A generic error containing just a message.
It implements [fmt::Display] and [fmt::Debug], and it can be converted from [io::Error] and [num::ParseIntError].

#### Example

```rust
use rdcl_aoc_helpers::parse_error;
use rdcl_aoc_helpers::error::ParseError;

fn example_with_params(param: u8) -> Result<(), ParseError> {
    if process(param) {
        Ok(())
    } else {
        Err(parse_error!("Failed to process param: {}", param))
    }
}

fn example_without_params() -> Result<(), ParseError> {
    if process() {
        Ok(())
    } else {
        Err(parse_error!("Failed to process"))
    }
}
```

## I/O operations

### `input::MultilineFromStr`

This trait is inspired by the [str::FromStr] trait, and allows you to parse input where data might span several lines.

#### Example

```rust
use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::input::MultilineFromStr;

pub struct Record {
    items: Vec<u8>,
}

impl MultilineFromStr for Record {
    type Err = ParseError;

    fn new() -> Self {
        Record {
            items: Vec::new(),
        }
    }

    fn indicates_new_record(&self, line: &str) -> bool {
        line.is_empty()
    }

    fn parse(&mut self, line: &str) -> Result<(), Self::Err> {
        if !line.is_empty() {
            self.items.push(line.parse::<u8>()?);
        }

        Ok(())
    }
}
```

### `input::WithReadLines`

This trait adds a `read_lines` method.
The purpose of this method is to read the lines from some source (e.g. a file), and then convert each line to a specific type.
As an argument, this method takes an exit code that should be used if processing the source fails, and it returns an iterator.
This trait has been implemented for [fs::File].

#### Example

```rust
use rdcl_aoc_helpers::input::WithReadLines;

fn main() {
    for item in File::open("./my-file.txt").read_lines::<u8>(1) {
        println!("Item: {}", item);
    }
}
```

### `input::WithReadMultiLines`

This trait adds a `read_multi_lines` method.
It's the equivalent of `input::WithReadLines`, but rather than depending on [str::FromStr], it depends on `input::MultilineFromStr`.

#### Example

```rust
use rdcl_aoc_helpers::input::WithReadMultiLines;

fn main() {
    for record in File::open("./my-file.txt").read_multi_lines::<Record>(1) {
        println!("Item: {:?}", record);
    }
}

#[derive(Debug)]
struct Record { /* ... */ }
impl MultilineFromStr for Record { /* ... */ }
```

### `input::WithAsRecords` & `input::WithAsMultilineRecords`

These traits allow you to easily convert an object to a vec of items of the required type.

#### Example

```rust
#[cfg(test)]
mod tests {
    use rdcl_aoc_helpers::input::WithAsRecords;
    use rdcl_aoc_helpers::input::WithAsMultilineRecords;

    use super::*;

    #[test]
    fn test_simple() {
        let input = vec!["1", "2", "3", "4", "5"]
            .as_records::<u8>()
            .unwrap();

        assert_eq!(input, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_multiline() {
        let input = vec!["1", "2", "", "3", "4", "", "5"]
            .as_multiline_records::<Record>()
            .unwrap();

        assert_eq!(
            input,
            vec![
                Record { items: vec![1, 2] },
                Record { items: vec![3, 4] },
                Record { items: vec![5] },
            ]
        );
    }
}
```

## Assembly machine

This module provides a machine that can process assembly-like instructions.
A machine consist of three parts:
* Instructions (must implement `MachineInstruction`)
* A register (must implement `MachineRegister`)
* An output receiver (must implement `OutputReceiver`)

When the machine is run, it will start executing the instructions one by one.
Each instruction specifies what the next instruction is (usually the next line, but you can also jump to some other spot).
As soon as the program counter reaches a non-existent instruction, the machine will halt.
Instructions may use the register to keep track of values, and may use the output receiver to send output to some (imaginary) output device, such as [an antenna](https://adventofcode.com/2016/day/25).
When running the machine, you also get to specify a pre-execute hook (`PreExecuteHook`).
For each instructions, the machine will call this hook before actually executing it.
This allows you to manipulate the normal flow of the program.
For example:
* [Apply optimizations](https://adventofcode.com/2016/day/23)
* [Detect loops](https://adventofcode.com/2020/day/8)

To more easily parse instructions from your input file, this library provides:
* `machine::instruction::Value` - Represents either a raw value or a reference to a register.
* `machine::instruction::ParsedMachineInstruction` - Represents a single parsed line. Allows you to easily access to command (`.get_command()`) or specific arguments, which will be parsed to the correct type (`.get_argument::<T>(idx)`).

### Example

```rust
use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::machine::hook::NoopHook;
use rdcl_aoc_helpers::machine::instruction::{MachineInstruction, ParsedMachineInstruction, Value};
use rdcl_aoc_helpers::machine::output_receiver::OutputReceiver;
use rdcl_aoc_helpers::machine::register::MachineRegister;
use rdcl_aoc_helpers::machine::Machine;

fn main() {
    let instructions = parse_input().unwrap();
    let mut machine = Machine::new_simple_machine(&instructions);
    machine.run(&mut NoopHook::default());

    println!("Final register state: {}", machine.register);
}

fn parse_input() -> Result<Vec<Instruction>, ParseError> {
    /* ... */
}

enum Instruction {
    Add(Value, Value, char),
    Jump(i64),
}

impl MachineInstruction for Instruction {
    fn execute<R: MachineRegister, O: OutputReceiver<R>>(
        &self,
        register: &mut R,
        _output_receiver: &mut O,
    ) -> i64 {
        match self {
            Instruction::Add(v1, v2, reg) => {
                register.write(*reg, v1.get(register), v2.get(register));
                1
            }
            Instruction::Jump(by) => by,
        }
    }

    fn from_parsed_machine_instruction(
        parsed: &ParsedMachineInstruction,
    ) -> Result<Self, ParseError> {
        match parsed.get_command() {
            "add" => Ok(Instruction::Add(
                parsed.get_argument(0)?,
                parsed.get_argument(1)?,
                parsed.get_argument(2)?
            )),
            "jmp" => Ok(Instruction::Jump(
                parsed.get_argument(0)?
            )),
            _ => Err(ParseError(format!("Unknown command: {}", parsed))),
        }
    }
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        <Self as MachineInstruction>::from_str(s)
    }
}
```


## Math

### `math::abs_diff` & `math::taxi_cab_*d`

The function `abs_diff` computes the absolute difference between two points.
The functions `taxi_cab_*d` (implemented for 2D, 3D and 4D) computes the taxi cab distance between two points.

#### Example

```rust
use rdcl_aoc_helpers::math::{abs_diff, taxi_cab_2d, taxi_cab_3d, taxi_cab_4d};

fn main() {
    println!("|1 - 5| = {}", abs_diff(1, 5));
    println!("|(1, 10) - (5, 3)| = {}", taxi_cab_2d((1, 10), (5, 3)));
    println!("|(1, 10, 2) - (5, 3, 4)| = {}", taxi_cab_3d((1, 10, 2), (5, 3, 4)));
    println!("|(1, 10, 2, 7) - (5, 3, 4, 2)| = {}", taxi_cab_4d((1, 10, 2, 7), (5, 3, 4, 2)));
}
```

### `math::gcd`

Computes the greatest common divisor of two numbers.

#### Example

```rust
use rdcl_aoc_helpers::math::gcd;

fn main() {
    let a = 35;
    let b = 49;
    println!("gcd({}, {}) = {}", a, b, gcd(a, b));
}
```

### `math::lcm`

Computes the least common multiple of two numbers.

#### Example

```rust
use rdcl_aoc_helpers::math::lcm;

fn main() {
    let a = 35;
    let b = 49;
    println!("lcm({}, {}) = {}", a, b, lcm(a, b));
}
```

### `math::solve_crt`

Solve the chinese remainder theorem for (n1, a1) and (n2, a2). We assume that:
* n1 and n2 are coprime
* n1 and n2 are no more than 63 bits (as they are converted to i64)

#### Example

```rust
use rdcl_aoc_helpers::math::solve_crt;

fn main() {
    println!("solve_crt((3, 1), (5, 4)) = {}", solve_crt((3, 1), (5, 4)));
}
```

### `math::bezout_coefficients`

Find t and s, such that ta + sb = gcd(p, q).

#### Example

```rust
use rdcl_aoc_helpers::math::bezout_coefficients;

fn main() {
    println!("bezout_coefficients(3, 4) = {}", bezout_coefficients(3, 4));
}
```

## Parts

### `part::Part`

This enum is useful if you need to explicitly refer to a part.
It implements [str::FromStr] and [fmt::Display], so you can easily convert to and from a string.

#### Example

```rust
use rdcl_aoc_helpers::part::Part;

fn main() {
    let part = "part 1".parse::<Part>().unwrap();
    println!("[{}] ...", part); // outputs "[part 1] ..."

    let part = Part::Two;
    println!("[{}] ...", part); // outputs "[part 2] ..."
}
```

## Search

### `search::Navigable`

Implement this trait to be able to use [A*] to find the shortest path between two points.


[aoc]: https://adventofcode.com
[A*]: https://en.wikipedia.org/wiki/A*_search_algorithm
[Result]: https://doc.rust-lang.org/std/result/enum.Result.html
[Option]: https://doc.rust-lang.org/std/option/enum.Option.html
[fmt::Debug]: https://doc.rust-lang.org/std/fmt/trait.Debug.html
[fmt::Display]: https://doc.rust-lang.org/std/fmt/trait.Display.html
[fs::File]: https://doc.rust-lang.org/std/fs/struct.File.html
[io::Error]: https://doc.rust-lang.org/std/io/struct.Error.html
[num::ParseIntError]: https://doc.rust-lang.org/std/num/struct.ParseIntError.html
[str::FromStr]: https://doc.rust-lang.org/std/str/trait.FromStr.html
