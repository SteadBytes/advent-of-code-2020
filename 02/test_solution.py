from typing import List

import pytest
from solution import part_1, part_2


@pytest.fixture
def password_list() -> List[str]:
    return ["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"]


def test_part_1(password_list):
    assert part_1(password_list) == 2


def test_part_2(password_list):
    assert part_2(password_list) == 1
