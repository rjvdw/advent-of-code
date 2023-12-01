# [Advent of Code]

[![Pipeline status][workflows-CI-badge]][actions]

My solutions for [Advent of Code].

## Structure

There is a workspace for every year, and there are workspaces in the [`lib`](lib/) directory with shared functionality.
The workspaces for the specific years all have a separate binary for every day.
All workspaces are prefixed with `rdcl_` to prevent naming collisions.

## Usage

```shell
# run the solution for a specific day
cargo run --package rdcl_aoc2022 --bin rdcl_aoc2022_day01

# run all tests for a specific year
cargo test --package rdcl_aoc2022

# run all tests for a specific day
cargo test --package rdcl_aoc2022 --bin rdcl_aoc2022_day01
```

To reduce the verbosity in these commands, there is also a helper script available.

```shell
# run the solution for a specific day
cargo run $(scripts/cargo-opts 2022 01)

# or using shorthand
cargo run $(scripts/cargo-opts 22 1)

# run all tests for a specific year
cargo test $(scripts/cargo-opts 22)

# run all tests for a specific day
cargo test $(scripts/cargo-opts 22 1)
```

[Advent of Code]: https://adventofcode.com/
[workflows-CI-badge]: https://github.com/rjvdw/advent-of-code/actions/workflows/ci-rust-v2.yml/badge.svg
[actions]: https://github.com/rjvdw/advent-of-code/actions/workflows/ci-rust-v2.yml

