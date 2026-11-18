from collections.abc import Callable
import math
import unittest


def fixed_point(f: Callable[[float], float], guess: float) -> float | None:
    for _ in range(1000):
        nxt = f(guess)
        if not math.isfinite(nxt):
            return None
        if abs(nxt - guess) < 1e-12:
            return nxt
        guess = nxt
    return None


class FixedPointTests(unittest.TestCase):
    def test_cosine_fixed_point(self) -> None:
        x = fixed_point(math.cos, 1.0)
        self.assertIsNotNone(x)
        assert x is not None
        self.assertLess(abs(x - 0.739085133215), 1e-10)


if __name__ == "__main__":
    print(f"cos(x)=x at {fixed_point(math.cos, 1.0)}")
