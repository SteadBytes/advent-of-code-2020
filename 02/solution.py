from __future__ import annotations

from typing import Iterable, TextIO, Tuple


def part_1(lines: Iterable[str]) -> int:
    return sum(1 if pw.count(l) in r else 0 for r, l, pw in map(parse_line, lines))


def parse_line(line: str) -> Tuple[range, str, str]:
    policy, pw = line.split(": ")
    valid_range, l = policy.split(" ")
    low, high = map(int, valid_range.split("-"))
    return range(low, high + 1), l, pw


def part_2():
    pass


def main(puzzle_input_f: TextIO):
    lines = [l.strip() for l in puzzle_input_f.readlines() if l]
    print("Part 1: ", part_1(lines))
    print("Part 2: ", part_2())


if __name__ == "__main__":
    import os

    from aocpy import input_cli

    base_dir = os.path.dirname(__file__)
    with input_cli(base_dir) as f:
        main(f)
