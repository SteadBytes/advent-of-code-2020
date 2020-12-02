from solution import part_1, part_2

import pytest


def test_part_1():
    password_list = ["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"]

    assert part_1(password_list) == 2


@pytest.mark.skip
def test_part_2():
    pass
