from __future__ import annotations

from operator import xor
from typing import Iterable, TextIO, Tuple


def part_1(lines: Iterable[str]) -> int:
    return sum(
        1 if low <= pw.count(l) <= high else 0
        for (low, high, l), pw in map(parse_line, lines)
    )


def part_2(lines: Iterable[str]) -> int:
    return sum(
        1 if xor(pw[i - 1] == l, pw[j - 1] == l) else 0
        for (i, j, l), pw in map(parse_line, lines)
    )


def parse_line(line: str) -> Tuple[Tuple[int, int, str], str]:
    policy, pw = line.split(": ")
    valid_range, l = policy.split(" ")
    low, high = map(int, valid_range.split("-"))
    return (low, high, l), pw


def main(puzzle_input_f: TextIO):
    lines = [l.strip() for l in puzzle_input_f.readlines() if l]
    print("Part 1: ", part_1(lines))
    print("Part 2: ", part_2(lines))


if __name__ == "__main__":
    import os

    from aocpy import input_cli

    base_dir = os.path.dirname(__file__)
    with input_cli(base_dir) as f:
        main(f)
