from __future__ import annotations

import math
from typing import Callable, Generic, TypeVar
import unittest

T = TypeVar("T")
U = TypeVar("U")


class Stream(Generic[T]):
    def __init__(self, head: T, tail_thunk: Callable[[], Stream[T]]) -> None:
        self.head = head
        self._thunk: Callable[[], Stream[T]] | None = tail_thunk
        self._forced: Stream[T] | None = None

    def cdr(self) -> Stream[T]:
        if self._forced is None:
            assert self._thunk is not None
            self._forced = self._thunk()
            self._thunk = None
        return self._forced

    def ref_at(self, n: int) -> T:
        cur = self
        for _ in range(n):
            cur = cur.cdr()
        return cur.head

    def map(self, fn: Callable[[T], U]) -> Stream[U]:
        return Stream(fn(self.head), lambda: self.cdr().map(fn))


def partial_sums(s: Stream[float]) -> Stream[float]:
    def go(cur: Stream[float], acc: float) -> Stream[float]:
        nxt = acc + cur.head
        return Stream(nxt, lambda: go(cur.cdr(), nxt))

    return go(s, 0.0)


def pi_summands(n: int = 1) -> Stream[float]:
    sign = 1.0 if ((n - 1) // 2) % 2 == 0 else -1.0
    return Stream(sign / n, lambda: pi_summands(n + 2))


def pi_stream() -> Stream[float]:
    return partial_sums(pi_summands()).map(lambda x: 4.0 * x)


def euler_transform(s: Stream[float]) -> Stream[float]:
    s0, s1, s2 = s.ref_at(0), s.ref_at(1), s.ref_at(2)
    denom = s0 - 2.0 * s1 + s2
    val = s2 if abs(denom) < 1e-18 else s2 - ((s2 - s1) ** 2) / denom
    return Stream(val, lambda: euler_transform(s.cdr()))


def make_tableau(transform: Callable[[Stream[float]], Stream[float]], s: Stream[float]) -> Stream[Stream[float]]:
    return Stream(s, lambda: make_tableau(transform, transform(s)))


def accelerated_sequence(transform: Callable[[Stream[float]], Stream[float]], s: Stream[float]) -> Stream[float]:
    return make_tableau(transform, s).map(lambda row: row.head)


def integers_from(n: int) -> Stream[int]:
    return Stream(n, lambda: integers_from(n + 1))


def merge_weighted(
    s1: Stream[tuple[int, int]],
    s2: Stream[tuple[int, int]],
    weight: Callable[[tuple[int, int]], int],
) -> Stream[tuple[int, int]]:
    if weight(s1.head) <= weight(s2.head):
        return Stream(s1.head, lambda: merge_weighted(s1.cdr(), s2, weight))
    return Stream(s2.head, lambda: merge_weighted(s1, s2.cdr(), weight))


def weighted_pairs(
    s: Stream[int],
    t: Stream[int],
    weight: Callable[[tuple[int, int]], int],
) -> Stream[tuple[int, int]]:
    return Stream(
        (s.head, t.head),
        lambda: merge_weighted(
            t.cdr().map(lambda x: (s.head, x)),
            weighted_pairs(s.cdr(), t.cdr(), weight),
            weight,
        ),
    )


def ramanujan_numbers(limit: int = 400) -> list[int]:
    weight = lambda p: p[0] ** 3 + p[1] ** 3  # noqa: E731
    cur = weighted_pairs(integers_from(1), integers_from(1), weight)
    found: list[int] = []
    prev_w = -1
    for _ in range(limit):
        w = weight(cur.head)
        if w == prev_w and w not in found:
            found.append(w)
        prev_w = w
        cur = cur.cdr()
    return found


def solve_ode(f: Callable[[float], float], y0: float, dt: float) -> Stream[float]:
    dy_ref: list[Stream[float]] = []

    def integral(acc: float) -> Stream[float]:
        def go(s: Stream[float], total: float) -> Stream[float]:
            nxt = total + s.head * dt
            return Stream(nxt, lambda: go(s.cdr(), nxt))

        return Stream(acc, lambda: go(dy_ref[0], acc))

    y = integral(y0)
    dy_ref.append(y.map(f))
    return y


def cesaro_pi_stream(seed: int) -> Stream[float]:
    def rand_stream(state: int) -> Stream[int]:
        nxt = (state * 6_364_136_223_846_793_005 + 1_442_695_040_888_963_407) & ((1 << 64) - 1)
        return Stream(nxt >> 16, lambda: rand_stream(nxt))

    def pairs(s: Stream[int]) -> Stream[bool]:
        s1 = s.cdr()
        return Stream(math.gcd(max(1, s.head), max(1, s1.head)) == 1, lambda: pairs(s1.cdr()))

    def mc(exp: Stream[bool], passed: int, failed: int) -> Stream[float]:
        p = passed + (1 if exp.head else 0)
        f = failed + (0 if exp.head else 1)
        return Stream(p / (p + f), lambda: mc(exp.cdr(), p, f))

    return mc(pairs(rand_stream(seed)), 0, 0).map(lambda p: math.sqrt(6.0 / p) if p > 0 else 0.0)


class StreamAccelerationPairsTest(unittest.TestCase):
    def test_tableau_acceleration_converges_fast(self) -> None:
        self.assertGreater(abs(pi_stream().ref_at(7) - math.pi), 0.05)
        self.assertLess(abs(accelerated_sequence(euler_transform, pi_stream()).ref_at(7) - math.pi), 1e-10)

    def test_ramanujan_ode_and_stream_mc(self) -> None:
        self.assertEqual(ramanujan_numbers()[:2], [1729, 4104])
        self.assertLess(abs(solve_ode(lambda y: y, 1.0, 0.001).ref_at(1000) - math.e), 0.002)
        self.assertLess(abs(cesaro_pi_stream(777).ref_at(10_000) - math.pi), 0.1)


if __name__ == "__main__":
    print("tableau pi[7] =", f"{accelerated_sequence(euler_transform, pi_stream()).ref_at(7):.12f}")
    print("Ramanujan[:2] =", ramanujan_numbers()[:2])
    print("ODE e ≈", f"{solve_ode(lambda y: y, 1.0, 0.001).ref_at(1000):.6f}")
