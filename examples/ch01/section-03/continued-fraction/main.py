from collections.abc import Callable
import math
import unittest


def continued_fraction(
    numerator: Callable[[int], float],
    denominator: Callable[[int], float],
    depth: int,
) -> float:
    tail = 0.0
    for i in range(depth, 0, -1):
        tail = numerator(i) / (denominator(i) + tail)
    return tail


class ContinuedFractionTests(unittest.TestCase):
    def test_converges_to_inverse_phi(self) -> None:
        value = continued_fraction(lambda _: 1.0, lambda _: 1.0, 24)
        self.assertLess(abs(value - (math.sqrt(5.0) - 1.0) / 2.0), 1e-9)


if __name__ == "__main__":
    inverse_phi = continued_fraction(lambda _: 1.0, lambda _: 1.0, 20)
    print(f"1/phi ≈ {inverse_phi:.10f}")
