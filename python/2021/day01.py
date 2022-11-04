#!/usr/bin/env python3
import sys


def main():
    nrs = parse_file(sys.argv[1])
    answer = count_increases(nrs, int(sys.argv[2]))
    print(answer)


def count_increases(nrs: list[int], window_size: int) -> int:
    if len(nrs) < window_size:
        return 0

    nr_increases = 0
    prev = sum(nrs[0:window_size])
    for i in range(1, len(nrs) - window_size + 1):
        cur = sum(nrs[i:i + window_size])
        if cur > prev:
            nr_increases += 1
        prev = cur

    return nr_increases


def parse_file(filename: str) -> list[int]:
    with open(filename) as file:
        return [int(line.rstrip()) for line in file]


if __name__ == "__main__":
    main()
