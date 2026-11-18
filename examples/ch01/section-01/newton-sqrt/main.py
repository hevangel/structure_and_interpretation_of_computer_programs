import math
import unittest


def square_root(x: float) -> float:
    assert math.isfinite(x) and x >= 0.0, "requires finite nonnegative input"
    if x == 0.0:
        return 0.0
    # Start away from zero so the Newton quotient never divides by zero.
    guess = x if x >= 1.0 else 1.0
    for _ in range(200):
        nxt = (guess + x / guess) / 2.0
        # Relative delta stops cleanly across magnitudes from 1e-12 to 1e12.
        if abs(nxt - guess) <= 1e-13 * max(abs(nxt), 1.0):
            return nxt
        guess = nxt
    return guess


class NewtonSqrtTests(unittest.TestCase):
    def test_approximate_roots(self) -> None:
        for x in (0.0, 0.01, 2.0, 9.0, 10_000.0):
            r = square_root(x)
            self.assertLessEqual(abs(r * r - x), 1e-9 * max(x, 1.0))


if __name__ == "__main__":
    print(f"sqrt(2) ≈ {square_root(2.0):.12f}")
