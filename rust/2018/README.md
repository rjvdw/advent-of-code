# [Advent of Code 2018](https://adventofcode.com/2018)

[![Pipeline status][workflows-CI-badge]][actions]

My answers for the 2018 [Advent of Code](https://adventofcode.com/2018).

## To Do

* For part 2 of day 2, it might be more efficient to construct a binary search tree.
* The solution for day 7 has quite some duplication.
* The MinMax struct from day 10 can be generalized further and then moved to the helpers. (Or maybe such a lib already exists?)
* Day 12 makes the bold assumption that the pattern will stabilize with a period of 1. If the pattern starts repeating with a period of 2, my solution will not detect this.
* The solution for day 22 is quite slow. Also, I could not use the A* implementation from the helpers, so I had to copy paste. Would be nice if this can be fixed.


[workflows-CI-badge]: https://github.com/rjvdw/advent-of-code/actions/workflows/ci-rust-2018.yml/badge.svg
[actions]: https://github.com/rjvdw/advent-of-code/actions/workflows/ci-rust-2018.yml
[day-11-spoilers-1]: https://www.reddit.com/r/adventofcode/comments/5hoia9/2018_day_11_solutions/db1v1ws
