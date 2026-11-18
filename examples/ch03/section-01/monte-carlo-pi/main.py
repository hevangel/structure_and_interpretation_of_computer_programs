from __future__ import annotations

import math
from typing import Callable
import unittest


class RandGen:
    def __init__(self, seed: int) -> None:
        self.state = seed

    def generate(self) -> int:
        self.state = (self.state * 6_364_136_223_846_793_005 + 1_442_695_040_888_963_407) & (
            (1 << 64) - 1
        )
        return self.state >> 16

    def reset(self, seed: int) -> int:
        self.state = seed
        return seed

    def unit_float(self) -> float:
        return (self.generate() & ((1 << 48) - 1)) / float(1 << 48)

    def random_in_range(self, low: float, high: float) -> float:
        return low + self.unit_float() * (high - low)


def monte_carlo(trials: int, experiment: Callable[[], bool]) -> float:
    assert trials > 0
    passed = sum(1 for _ in range(trials) if experiment())
    return passed / trials


def estimate_pi_cesaro(rng: RandGen, trials: int) -> float:
    p = monte_carlo(
        trials,
        lambda: math.gcd(max(1, rng.generate()), max(1, rng.generate())) == 1,
    )
    return math.sqrt(6.0 / p)


def estimate_integral(
    rng: RandGen,
    pred: Callable[[float, float], bool],
    x1: float,
    x2: float,
    y1: float,
    y2: float,
    trials: int,
) -> float:
    frac = monte_carlo(
        trials,
        lambda: pred(rng.random_in_range(x1, x2), rng.random_in_range(y1, y2)),
    )
    return frac * (x2 - x1) * (y2 - y1)


class MonteCarloPiTest(unittest.TestCase):
    def test_reset_determinism_and_pi_estimates(self) -> None:
        rng = RandGen(12345)
        seq1 = [rng.generate() for _ in range(5)]
        rng.reset(12345)
        seq2 = [rng.generate() for _ in range(5)]
        self.assertEqual(seq1, seq2)

        self.assertLess(abs(estimate_pi_cesaro(rng, 40_000) - math.pi), 0.1)
        area = estimate_integral(rng, lambda x, y: x * x + y * y <= 1.0, -1.0, 1.0, -1.0, 1.0, 40_000)
        self.assertLess(abs(area - math.pi), 0.05)


if __name__ == "__main__":
    rng = RandGen(42)
    print(f"Cesaro pi ≈ {estimate_pi_cesaro(rng, 20_000):.4f}")
    rng.reset(42)
    print(
        f"Unit-disk pi ≈ {estimate_integral(rng, lambda x, y: x*x + y*y <= 1.0, -1.0, 1.0, -1.0, 1.0, 20_000):.4f}"
    )
