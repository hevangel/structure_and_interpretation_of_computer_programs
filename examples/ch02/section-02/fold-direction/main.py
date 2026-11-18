from collections.abc import Callable
import unittest


def fold_left(items: list[int], initial: int, f: Callable[[int, int], int]) -> int:
    acc = initial
    for item in items:
        acc = f(acc, item)
    return acc


def fold_right(items: list[int], initial: int, f: Callable[[int, int], int]) -> int:
    suffix = initial
    for item in reversed(items):
        suffix = f(item, suffix)
    return suffix


class FoldDirectionTests(unittest.TestCase):
    def test_subtraction_is_not_associative(self) -> None:
        values = [1, 2, 3]
        self.assertEqual(fold_left(values, 0, lambda a, b: a - b), -6)
        self.assertEqual(fold_right(values, 0, lambda a, b: a - b), 2)
        self.assertEqual(
            fold_left(values, 0, lambda a, b: a + b),
            fold_right(values, 0, lambda a, b: a + b),
        )


if __name__ == "__main__":
    values = [1, 2, 3]
    print(
        f"left {fold_left(values, 0, lambda a, b: a - b)}, "
        f"right {fold_right(values, 0, lambda a, b: a - b)}"
    )
