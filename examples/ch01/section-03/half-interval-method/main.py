from collections.abc import Callable
import math
import unittest


def root(
    f: Callable[[float], float], low: float, high: float, tolerance: float
) -> float | None:
    assert tolerance > 0.0 and low < high
    left = f(low)
    right = f(high)
    if not (math.isfinite(left) and math.isfinite(right)):
        return None
    if left == 0.0:
        return low
    if right == 0.0:
        return high
    if (left >= 0.0) == (right >= 0.0):
        return None
    for _ in range(200):
        mid = low + (high - low) / 2.0
        value = f(mid)
        if value == 0.0 or (high - low) <= tolerance:
            return mid
        if not math.isfinite(value):
            return None
        if (value >= 0.0) == (left >= 0.0):
            low = mid
            left = value
        else:
            high = mid
    return low + (high - low) / 2.0


class HalfIntervalMethodTests(unittest.TestCase):
    def test_bracketing_is_preserved(self) -> None:
        value = root(lambda x: x * x * x - 2.0, 1.0, 2.0, 1e-12)
        self.assertIsNotNone(value)
        assert value is not None
        self.assertLess(abs(value - 2.0 ** (1.0 / 3.0)), 1e-10)
        self.assertIsNone(root(lambda x: x * x + 1.0, 0.0, 2.0, 1e-12))
        self.assertEqual(root(lambda x: x - 3.0, 3.0, 4.0, 1e-12), 3.0)


if __name__ == "__main__":
    print(f"cuberoot(2)≈{root(lambda x: x * x * x - 2.0, 1.0, 2.0, 1e-12)}")
