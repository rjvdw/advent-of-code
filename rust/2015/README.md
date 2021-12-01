# [Advent of Code 2015](https://adventofcode.com/2015)

[![Pipeline status][workflows-CI-badge]][actions]

My answers for the 2015 [Advent of Code](https://adventofcode.com/2015).

## To Do

* For day 4, it might be smarter to compute the MD5 hash myself. Need to investigate...
* Day 6 could use some additional unit tests. Also, it's a bit slow.
* Day 9 could be made a bit faster by computing the shortest and the longest path in one go. Or maybe just by using memoization.
* There might be a more memory efficient implementation for the grid from day 18 than a `Vec<bool>`.
* For day 24, I should verify if the grouping that is found for part 2 is actually valid (i.e. that the remaining packages can be split up in three equal groups).


[workflows-ci-rust-badge]: https://github.com/rjvdw/advent-of-code/actions/workflows/ci-2015.yml/badge.svg
[actions]: https://github.com/rjvdw/advent-of-code/actions/workflows/ci-rust-2015.yml
