from __future__ import annotations

from typing import Callable, Generic, TypeVar
import unittest

K = TypeVar("K")
V = TypeVar("V")


class Table(Generic[K, V]):
    """Recursive multi-key table with pluggable equality predicate (Exercises 3.24-3.25)."""

    def __init__(self, same_key: Callable[[K, K], bool] | None = None) -> None:
        self.same_key: Callable[[K, K], bool] = same_key or (lambda a, b: a == b)
        self.value: V | None = None
        self.has_value = False
        self.entries: list[tuple[K, Table[K, V]]] = []

    def _assoc(self, key: K) -> Table[K, V] | None:
        for k, sub in self.entries:
            if self.same_key(k, key):
                return sub
        return None

    def insert(self, keys: list[K], value: V) -> None:
        if not keys:
            self.value = value
            self.has_value = True
            return
        head, tail = keys[0], keys[1:]
        sub = self._assoc(head)
        if sub is None:
            sub = Table(self.same_key)
            self.entries.append((head, sub))
        sub.insert(tail, value)

    def lookup(self, keys: list[K]) -> V | None:
        if not keys:
            return self.value if self.has_value else None
        sub = self._assoc(keys[0])
        return None if sub is None else sub.lookup(keys[1:])


def memoize(fn: Callable[[Callable[[int], int], int], int]) -> tuple[Callable[[int], int], list[int]]:
    cache: dict[int, int] = {}
    calls = [0]

    def wrapper(n: int) -> int:
        if n in cache:
            return cache[n]
        calls[0] += 1
        res = fn(wrapper, n)
        cache[n] = res
        return res

    return wrapper, calls


class LocalTablesMemoizeTest(unittest.TestCase):
    def test_multidimensional_table_with_tolerance(self) -> None:
        tbl: Table[float, int] = Table(lambda a, b: abs(a - b) < 0.01)
        tbl.insert([1.0, 2.0, 3.0], 42)
        tbl.insert([1.0, 2.5, 3.0], 99)
        self.assertEqual(tbl.lookup([1.005, 1.995, 3.001]), 42)
        self.assertEqual(tbl.lookup([1.0, 2.5, 3.0]), 99)
        self.assertIsNone(tbl.lookup([1.0, 2.2, 3.0]))

    def test_memoize_evaluates_each_n_once(self) -> None:
        memo_fib, calls = memoize(lambda rec, n: n if n <= 1 else rec(n - 1) + rec(n - 2))
        self.assertEqual(memo_fib(30), 832_040)
        self.assertEqual(calls[0], 31)
        self.assertEqual(memo_fib(30), 832_040)
        self.assertEqual(calls[0], 31)


if __name__ == "__main__":
    grid: Table[float, str] = Table(lambda a, b: abs(a - b) < 0.05)
    grid.insert([1.0, 2.0], "point-A")
    print("lookup [1.02, 1.98] =", grid.lookup([1.02, 1.98]))
    fib, calls = memoize(lambda rec, n: n if n <= 1 else rec(n - 1) + rec(n - 2))
    print(f"memo_fib(30) = {fib(30)} in {calls[0]} calls")
