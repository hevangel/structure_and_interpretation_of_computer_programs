import math
import unittest


def sine(angle: float) -> tuple[float, int]:
    def approximate(x: float, depth: int) -> tuple[float, int]:
        if abs(x) < 0.001:
            return x, depth
        small, levels = approximate(x / 3.0, depth + 1)
        return 3.0 * small - 4.0 * small * small * small, levels

    return approximate(angle, 0)


class SineTripleAngleTests(unittest.TestCase):
    def test_reduced_angles_are_accurate(self) -> None:
        for x in (-12.15, -2.0, -0.5, 0.0, 0.5, 2.0, 12.15):
            estimate, _ = sine(x)
            self.assertLess(abs(estimate - math.sin(x)), 0.002)
        self.assertGreater(sine(12.15)[1], 1)


if __name__ == "__main__":
    value, levels = sine(12.15)
    print(f"sin(12.15)≈{value:.6f}, reductions={levels}")
