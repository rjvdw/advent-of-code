# [Advent of Code 2019](https://adventofcode.com/2019)

[![Pipeline status][workflows-CI-badge]][actions]

My answers for the 2019 [Advent of Code](https://adventofcode.com/2019).

## To do

* Currently, my solution for day 12 makes the assumption that the repeating state will be the beginning state.
* The rendering for the arcade of day 13 can be made more efficient, by only rendering the spaces that actually have updated.
* For day 20, I opted to make some assumptions to make parsing the input easier. Also, there is some duplication between parts 1 and 2, might be possible to make this a bit more generic.
* For day 22, there are a lot of conversions between i64 and BigInt going on. The main reason for this, is that I only implemented the extend GCD for i64, not for BigInt.

[workflows-CI-badge]: https://github.com/rjvdw/advent-of-code/actions/workflows/ci-rust-2019.yml/badge.svg
[actions]: https://github.com/rjvdw/advent-of-code/actions/workflows/ci-rust-2019.yml
