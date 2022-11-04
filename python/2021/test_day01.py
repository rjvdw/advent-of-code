from day01 import count_increases


def test_with_window_1():
    nrs = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263]
    assert count_increases(nrs, 1) == 7


def test_with_window_3():
    nrs = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263]
    assert count_increases(nrs, 3) == 5
