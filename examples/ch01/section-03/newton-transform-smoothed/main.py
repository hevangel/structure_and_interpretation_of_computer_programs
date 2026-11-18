from __future__ import annotations

import math
from typing import Callable
import unittest

RealFn = Callable[[float], float]


def fixed_point(f: RealFn, guess: float = 1.0, tol: float = 1e-11) -> float:
    for _ in range(500):
        nxt = f(guess)
        if abs(nxt - guess) < tol:
            return nxt
        guess = nxt
    return guess


def average_damp(f: RealFn) -> RealFn:
    return lambda x: 0.5 * (x + f(x))


def deriv(g: RealFn, dx: float = 1e-6) -> RealFn:
    return lambda x: (g(x + dx) - g(x)) / dx


def newton_transform(g: RealFn) -> RealFn:
    dg = deriv(g)
    return lambda x: x - g(x) / dg(x)


def newtons_method(g: RealFn, guess: float = 1.0) -> float:
    return fixed_point(newton_transform(g), guess)


def compose(f: RealFn, g: RealFn) -> RealFn:
    return lambda x: f(g(x))


def repeated(f: RealFn, n: int) -> RealFn:
    acc: RealFn = lambda x: x
    for _ in range(n):
        acc = compose(f, acc)
    return acc


def smooth(f: RealFn, dx: float = 0.1) -> RealFn:
    return lambda x: (f(x - dx) + f(x) + f(x + dx)) / 3.0


def nth_root(x: float, n: int) -> float:
    assert x >= 0.0 and n >= 1
    damps = max(1, int(math.floor(math.log2(n))))
    damped: RealFn = lambda y: x / (y ** (n - 1))
    for _ in range(damps):
        damped = average_damp(damped)
    return fixed_point(damped, 1.0)


def iterative_improve(good_enough: Callable[[float], bool], improve: RealFn) -> RealFn:
    def run(guess: float) -> float:
        for _ in range(500):
            if good_enough(guess):
                return guess
            guess = improve(guess)
        return guess

    return run


class FirstClassTransformTest(unittest.TestCase):
    def test_newtons_method_cubic(self) -> None:
        root = newtons_method(lambda x: x**3 - 2.0 * x**2 + x - 2.0, 2.5)
        self.assertAlmostEqual(root, 2.0, places=9)

    def test_nth_root_converges(self) -> None:
        self.assertAlmostEqual(nth_root(128.0, 7), 2.0, places=9)
        self.assertAlmostEqual(nth_root(81.0, 4), 3.0, places=9)

    def test_smooth_repeated_and_iterative_improve(self) -> None:
        spike = lambda x: 6.0 if abs(x) < 1e-9 else 0.0  # noqa: E731
        self.assertAlmostEqual(smooth(spike, 0.1)(0.0), 2.0, places=12)
        self.assertEqual(repeated(lambda x: x + 1.0, 3)(4.0), 7.0)
        sqrt_fn = iterative_improve(lambda g: abs(g * g - 25.0) < 1e-11, lambda g: 0.5 * (g + 25.0 / g))
        self.assertAlmostEqual(sqrt_fn(1.0), 5.0, places=9)


if __name__ == "__main__":
    print(f"cubic root = {newtons_method(lambda x: x**3 + x**2 + x - 3.0, 1.0):.9f}")
    print(f"128^(1/7) = {nth_root(128.0, 7):.9f}")
