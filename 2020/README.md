# [Advent of Code 2020](https://adventofcode.com/2020)

[![Pipeline status][workflows-CI-badge]][actions]

My answers for the 2020 [Advent of Code](https://adventofcode.com/2020).

## Usage

Run the solution for a specific day:

```bash
cargo run --package day05 day05/input
```

Run the tests:

```bash
cargo test
```

Run the tests for a specific day:

```bash
cargo test --package day04
```

Format your code:

```bash
cargo fmt
```

Check your code for common mistakes/code smells:

```bash
cargo clippy
```

## To Do

* For day 19, I feel the solution could be way faster. It currently runs in about 200ms on my laptop, but I think it can be done in less than 100ms.
* For day 20, the solution can be cleaned up a little bit.
  * Actually mark the dragons in the image.
  * Maybe do something clever, so not all rotations have to be marked explicitely.


[workflows-CI-badge]: https://github.com/rjvdw/advent-of-code/workflows/CI%202020/badge.svg
[actions]: https://github.com/rjvdw/advent-of-code/actions?query=workflow%3A%22CI+2020%22
