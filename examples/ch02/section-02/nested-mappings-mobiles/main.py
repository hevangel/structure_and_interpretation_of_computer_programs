from __future__ import annotations

from dataclasses import dataclass
from typing import Callable, TypeVar
import unittest

T = TypeVar("T")
U = TypeVar("U")


@dataclass(frozen=True)
class Branch:
    length: int
    structure: int | Mobile

    def weight(self) -> int:
        return self.structure if isinstance(self.structure, int) else self.structure.total_weight()

    def is_balanced(self) -> bool:
        return True if isinstance(self.structure, int) else self.structure.is_balanced()


@dataclass(frozen=True)
class Mobile:
    left: Branch
    right: Branch

    def total_weight(self) -> int:
        return self.left.weight() + self.right.weight()

    def is_balanced(self) -> bool:
        return (
            self.left.length * self.left.weight() == self.right.length * self.right.weight()
            and self.left.is_balanced()
            and self.right.is_balanced()
        )


def fold_right(op: Callable[[T, U], U], initial: U, seq: list[T]) -> U:
    if not seq:
        return initial
    return op(seq[0], fold_right(op, initial, seq[1:]))


def fold_left(op: Callable[[U, T], U], initial: U, seq: list[T]) -> U:
    acc = initial
    for item in seq:
        acc = op(acc, item)
    return acc


def flatmap(items: list[T], fn: Callable[[T], list[U]]) -> list[U]:
    return [out for item in items for out in fn(item)]


def is_prime(n: int) -> bool:
    if n < 2:
        return False
    d = 2
    while d * d <= n:
        if n % d == 0:
            return False
        d += 1
    return True


def prime_sum_pairs(n: int) -> list[tuple[int, int, int]]:
    pairs = flatmap(list(range(1, n + 1)), lambda i: [(i, j) for j in range(1, i)])
    return [(i, j, i + j) for i, j in pairs if is_prime(i + j)]


def permutations(items: list[T]) -> list[list[T]]:
    if not items:
        return [[]]
    return flatmap(
        items,
        lambda x: [[x, *p] for p in permutations([y for y in items if y != x])],
    )


class NestedMappingsMobilesTest(unittest.TestCase):
    def test_mobile_balance(self) -> None:
        inner = Mobile(Branch(2, 3), Branch(3, 2))
        outer = Mobile(Branch(2, inner), Branch(5, 2))
        self.assertEqual(outer.total_weight(), 7)
        self.assertTrue(outer.is_balanced())

    def test_folds_and_flatmap(self) -> None:
        seq = [1, 2, 3, 4]
        self.assertNotEqual(
            fold_right(lambda a, b: a - b, 0, seq),
            fold_left(lambda a, b: a - b, 0, seq),
        )
        self.assertEqual(
            fold_right(lambda a, b: a + b, 0, seq),
            fold_left(lambda a, b: a + b, 0, seq),
        )
        self.assertEqual(prime_sum_pairs(4), [(2, 1, 3), (3, 2, 5), (4, 1, 5), (4, 3, 7)])
        self.assertEqual(len(permutations([1, 2, 3, 4])), 24)


if __name__ == "__main__":
    print("prime_sum_pairs(5) =", prime_sum_pairs(5))
    print("permutations([1, 2, 3]) =", permutations([1, 2, 3]))
