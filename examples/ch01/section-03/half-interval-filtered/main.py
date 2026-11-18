from __future__ import annotations

import math
from typing import Callable, TypeVar
import unittest

T = TypeVar("T")


def filtered_accumulate(
    combiner: Callable[[T, T], T],
    null_value: T,
    term: Callable[[int], T],
    a: int,
    next_fn: Callable[[int], int],
    b: int,
    predicate: Callable[[int], bool],
) -> T:
    acc = null_value
    cur = a
    while cur <= b:
        if predicate(cur):
            acc = combiner(acc, term(cur))
        cur = next_fn(cur)
    return acc


def is_prime(n: int) -> bool:
    if n < 2:
        return False
    d = 2
    while d * d <= n:
        if n % d == 0:
            return False
        d += 1
    return True


def half_interval_method(f: Callable[[float], float], a: float, b: float, tol: float = 1e-12) -> float:
    fa, fb = f(a), f(b)
    if fa * fb > 0.0:
        raise ValueError("endpoints must have opposite signs")
    neg, pos = (a, b) if fa < 0.0 else (b, a)
    for _ in range(200):
        mid = 0.5 * (neg + pos)
        if abs(pos - neg) < tol:
            return mid
        val = f(mid)
        if abs(val) < tol * 0.1:
            return mid
        if val > 0.0:
            pos = mid
        else:
            neg = mid
    return 0.5 * (neg + pos)


def fixed_point_damped(f: Callable[[float], float], guess: float = 1.0, tol: float = 1e-12) -> float:
    for _ in range(500):
        nxt = 0.5 * (guess + f(guess))
        if abs(nxt - guess) < tol:
            return nxt
        guess = nxt
    return guess


def cont_frac(n: Callable[[int], float], d: Callable[[int], float], k: int) -> float:
    acc = 0.0
    for i in range(k, 0, -1):
        acc = n(i) / (d(i) + acc)
    return acc


def euler_e(k: int) -> float:
    return 2.0 + cont_frac(
        lambda _i: 1.0,
        lambda i: float(2 * ((i + 1) // 3)) if i % 3 == 2 else 1.0,
        k,
    )


def tan_cf(x: float, k: int) -> float:
    return cont_frac(
        lambda i: x if i == 1 else -x * x,
        lambda i: float(2 * i - 1),
        k,
    )


class HalfIntervalFilteredTest(unittest.TestCase):
    def test_filtered_accumulate(self) -> None:
        self.assertEqual(
            filtered_accumulate(lambda a, b: a + b, 0, lambda x: x * x, 1, lambda x: x + 1, 10, is_prime),
            87,
        )
        self.assertEqual(
            filtered_accumulate(
                lambda a, b: a * b, 1, lambda x: x, 1, lambda x: x + 1, 9, lambda i: math.gcd(i, 10) == 1
            ),
            189,
        )

    def test_roots_and_continued_fractions(self) -> None:
        pi = half_interval_method(math.sin, 2.0, 4.0)
        self.assertAlmostEqual(pi, math.pi, places=10)
        with self.assertRaises(ValueError):
            half_interval_method(lambda x: x * x + 1.0, 0.0, 2.0)
        self.assertAlmostEqual(fixed_point_damped(lambda y: 2.0 / y), math.sqrt(2.0), places=10)
        self.assertAlmostEqual(euler_e(20), math.e, places=12)
        self.assertAlmostEqual(tan_cf(0.7, 20), math.tan(0.7), places=12)


if __name__ == "__main__":
    print(f"pi via half-interval = {half_interval_method(math.sin, 2.0, 4.0):.12f}")
    print(f"euler_e(20) = {euler_e(20):.12f}, tan_cf(0.7, 20) = {tan_cf(0.7, 20):.12f}")
