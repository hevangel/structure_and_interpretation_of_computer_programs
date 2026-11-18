from __future__ import annotations

from typing import Callable, Generic, TypeVar
import unittest

T = TypeVar("T")
U = TypeVar("U")


class Stream(Generic[T]):
    """Memoized delay/force stream: tail thunk executes at most once."""

    EMPTY: Stream[object]

    def __init__(self, head: T, tail_thunk: Callable[[], Stream[T]]) -> None:
        self._empty = False
        self.head = head
        self._thunk: Callable[[], Stream[T]] | None = tail_thunk
        self._forced: Stream[T] | None = None

    @classmethod
    def empty(cls) -> Stream[T]:
        obj = cls.__new__(cls)
        obj._empty = True
        return obj

    def is_empty(self) -> bool:
        return self._empty

    def cdr(self) -> Stream[T]:
        assert not self._empty
        if self._forced is None:
            assert self._thunk is not None
            self._forced = self._thunk()
            self._thunk = None
        return self._forced

    def take(self, n: int) -> list[T]:
        out: list[T] = []
        cur = self
        for idx in range(n):
            if cur.is_empty():
                break
            out.append(cur.head)
            if idx + 1 < n:
                cur = cur.cdr()
        return out

    def map(self, fn: Callable[[T], U]) -> Stream[U]:
        if self.is_empty():
            return Stream.empty()
        return Stream(fn(self.head), lambda: self.cdr().map(fn))

    def filter(self, pred: Callable[[T], bool]) -> Stream[T]:
        cur = self
        while not cur.is_empty():
            if pred(cur.head):
                tail = cur.cdr()
                return Stream(cur.head, lambda t=tail: t.filter(pred))
            cur = cur.cdr()
        return Stream.empty()


def add_streams(a: Stream[float], b: Stream[float]) -> Stream[float]:
    if a.is_empty() or b.is_empty():
        return Stream.empty()
    return Stream(a.head + b.head, lambda: add_streams(a.cdr(), b.cdr()))


def scale_stream(s: Stream[float], factor: float) -> Stream[float]:
    return s.map(lambda x: x * factor)


def integers_from(n: int) -> Stream[int]:
    return Stream(n, lambda: integers_from(n + 1))


def sieve(s: Stream[int]) -> Stream[int]:
    p = s.head
    return Stream(p, lambda: sieve(s.cdr().filter(lambda x, p=p: x % p != 0)))


def merge_streams(s1: Stream[int], s2: Stream[int]) -> Stream[int]:
    if s1.is_empty():
        return s2
    if s2.is_empty():
        return s1
    if s1.head < s2.head:
        return Stream(s1.head, lambda: merge_streams(s1.cdr(), s2))
    if s1.head > s2.head:
        return Stream(s2.head, lambda: merge_streams(s1, s2.cdr()))
    return Stream(s1.head, lambda: merge_streams(s1.cdr(), s2.cdr()))


def hamming_stream() -> Stream[int]:
    ref: list[Stream[int]] = []
    s = Stream(
        1,
        lambda: merge_streams(
            ref[0].map(lambda x: x * 2),
            merge_streams(ref[0].map(lambda x: x * 3), ref[0].map(lambda x: x * 5)),
        ),
    )
    ref.append(s)
    return s


def integrate_series(s: Stream[float]) -> Stream[float]:
    def go(cur: Stream[float], n: int) -> Stream[float]:
        if cur.is_empty():
            return Stream.empty()
        return Stream(cur.head / n, lambda: go(cur.cdr(), n + 1))

    return go(s, 1)


def mul_series(s1: Stream[float], s2: Stream[float]) -> Stream[float]:
    return Stream(
        s1.head * s2.head,
        lambda: add_streams(scale_stream(s2.cdr(), s1.head), mul_series(s1.cdr(), s2)),
    )


def invert_unit_series(s: Stream[float]) -> Stream[float]:
    ref: list[Stream[float]] = []
    inv = Stream(1.0, lambda: scale_stream(mul_series(s.cdr(), ref[0]), -1.0))
    ref.append(inv)
    return inv


def div_series(num: Stream[float], den: Stream[float]) -> Stream[float]:
    d0 = den.head
    assert abs(d0) > 1e-12
    inv = invert_unit_series(scale_stream(den, 1.0 / d0))
    return scale_stream(mul_series(num, inv), 1.0 / d0)


def sine_and_cosine_series() -> tuple[Stream[float], Stream[float]]:
    sin_ref: list[Stream[float]] = []
    cos_ref: list[Stream[float]] = []
    cos_s = Stream(1.0, lambda: scale_stream(integrate_series(sin_ref[0]), -1.0))
    sin_s = Stream(0.0, lambda: integrate_series(cos_ref[0]))
    sin_ref.append(sin_s)
    cos_ref.append(cos_s)
    return sin_s, cos_s


class MemoizedStreamsSeriesTest(unittest.TestCase):
    def test_tail_thunk_memoized_once(self) -> None:
        forces = [0]

        def counted(n: int) -> Stream[int]:
            def nxt() -> Stream[int]:
                forces[0] += 1
                return counted(n + 1)

            return Stream(n, nxt)

        s = counted(0)
        self.assertEqual(s.take(6), [0, 1, 2, 3, 4, 5])
        self.assertEqual(forces[0], 5)
        self.assertEqual(s.take(6), [0, 1, 2, 3, 4, 5])
        self.assertEqual(forces[0], 5)

    def test_sieve_and_hamming(self) -> None:
        self.assertEqual(sieve(integers_from(2)).take(10), [2, 3, 5, 7, 11, 13, 17, 19, 23, 29])
        self.assertEqual(hamming_stream().take(12), [1, 2, 3, 4, 5, 6, 8, 9, 10, 12, 15, 16])

    def test_trig_power_series_identity(self) -> None:
        sin_s, cos_s = sine_and_cosine_series()
        identity = add_streams(mul_series(sin_s, sin_s), mul_series(cos_s, cos_s)).take(13)
        self.assertAlmostEqual(identity[0], 1.0)
        for c in identity[1:]:
            self.assertAlmostEqual(c, 0.0, places=12)
        tan_c = div_series(sin_s, cos_s).take(6)
        for got, exp in zip(tan_c, [0.0, 1.0, 0.0, 1.0 / 3.0, 0.0, 2.0 / 15.0]):
            self.assertAlmostEqual(got, exp, places=12)


if __name__ == "__main__":
    print("primes[:10] =", sieve(integers_from(2)).take(10))
    print("hamming[:12] =", hamming_stream().take(12))
    sin_s, cos_s = sine_and_cosine_series()
    print("tan coeffs[:6] =", div_series(sin_s, cos_s).take(6))
