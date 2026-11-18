from __future__ import annotations

from dataclasses import dataclass
from typing import Callable, Generic, TypeVar
import unittest

T = TypeVar("T")


@dataclass(frozen=True)
class Done(Generic[T]):
    value: T


@dataclass(frozen=True)
class Call(Generic[T]):
    next_fn: Callable[[], Done[T] | Call[T]]


Bounce = Done[T] | Call[T]


def trampoline(step: Bounce[T]) -> T:
    while isinstance(step, Call):
        step = step.next_fn()
    return step.value


def tail_sum(n: int, acc: int) -> Bounce[int]:
    if n == 0:
        return Done(acc)
    return Call(lambda: tail_sum(n - 1, acc + n))


def main() -> None:
    print(f"sum(1..100000) = {trampoline(tail_sum(100_000, 0))}")


class TestJavascriptTailCalls(unittest.TestCase):
    def test_deep_tail_recursion_without_stack_growth(self) -> None:
        self.assertEqual(trampoline(tail_sum(100_000, 0)), 5_000_050_000)
        self.assertEqual(trampoline(tail_sum(0, 17)), 17)


if __name__ == "__main__":
    main()
