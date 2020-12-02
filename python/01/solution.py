"""
itertools feels like cheating...

This solution assumes that there is *exactly one* set of entries for each part
(pair or triple). A more robust solution would cover these cases explicitly.
"""
from __future__ import annotations

from itertools import combinations
from functools import reduce
from operator import mul
from typing import Iterable, TextIO


def part_1(entries: Iterable[int]) -> int:
    return mul(*next(c for c in combinations(entries, 2) if sum(c) == 2020))


def part_2(entries: Iterable[int]) -> int:
    return reduce(mul, next(c for c in combinations(entries, 3) if sum(c) == 2020))


def main(puzzle_input_f: TextIO):
    entries = [int(l.strip()) for l in puzzle_input_f.readlines() if l]
    print("Part 1: ", part_1(entries))
    print("Part 2: ", part_2(entries))


if __name__ == "__main__":
    import os

    from aocpy import input_cli

    base_dir = os.path.dirname(__file__)
    with input_cli(base_dir) as f:
        main(f)
