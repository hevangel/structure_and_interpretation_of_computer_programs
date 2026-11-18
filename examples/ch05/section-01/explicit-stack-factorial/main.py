from __future__ import annotations

import unittest


def factorial(n: int) -> tuple[int, int]:
    stack: list[int] = []
    current = n
    # Descend into the recurrence, preserving pending multiplications.
    while current > 1:
        stack.append(current)
        current -= 1
    max_depth = len(stack)
    value = 1
    while stack:
        value *= stack.pop()
    return value, max_depth


def main() -> None:
    print(f"10! and stack depth: {factorial(10)}")


class TestExplicitStackFactorial(unittest.TestCase):
    def test_deferred_multiplications(self) -> None:
        self.assertEqual(factorial(0), (1, 0))
        self.assertEqual(factorial(1), (1, 0))
        self.assertEqual(factorial(5), (120, 4))
        self.assertEqual(factorial(10)[0], 3_628_800)


if __name__ == "__main__":
    main()
