# [Advent of Code 2016](https://adventofcode.com/2016)

[![Pipeline status][workflows-CI-badge]][actions]

My answers for the 2016 [Advent of Code](https://adventofcode.com/2016).

## To Do

* I feel the solution to day 11 could probably be optimized further. See [this reddit post][day-11-spoilers-1] for some ideas.
* Both day 5 and day 14 require you to compute a lot of MD5 hashes. Surely there must be a faster way to do this... Also, some tests for these days have been disabled, because they run very slow unless you specify `--release`. Probably need to do something with cargo profiles to fix this.
* For day 21 I had to do some hardcoding, to get the reverse of the "rotate based on" instruction. I am not sure if this can be done without doing this, as this operation cannot be reversed if the input has a length of greater than 8.
* For day 22, I feel that the solution is not yet completely robust. Specifically, the second `filter` in `get_neighbours` feels like it could fail in specific edge cases.
* For day 24, I feel that there are some heuristics that can be applied on the `flood_fill` method. Right now I am doing an exhaustive search, but I think some branches can be ignored.
* I should really move the implementation of A* to the rust helpers...


[workflows-CI-badge]: https://github.com/rjvdw/advent-of-code/workflows/CI%202016/badge.svg
[actions]: https://github.com/rjvdw/advent-of-code/actions?query=workflow%3A%22CI+2016%22
[day-11-spoilers-1]: https://www.reddit.com/r/adventofcode/comments/5hoia9/2016_day_11_solutions/db1v1ws
