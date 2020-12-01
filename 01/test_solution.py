from solution import part_1, part_2


def test_part_1():
    entries = [
        1721,
        979,
        366,
        299,
        675,
        1456,
    ]

    assert part_1(entries) == 514579


def test_part_2():
    entries = [
        1721,
        979,
        366,
        299,
        675,
        1456,
    ]

    assert part_2(entries) == 241861950
