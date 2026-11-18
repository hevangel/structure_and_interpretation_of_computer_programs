from collections.abc import Callable
import unittest


def pair(first_val: int, second_val: int) -> Callable[[bool], int]:
    # The selector is the only interface for reading a functional pair.
    return lambda select_first: first_val if select_first else second_val


def first(p: Callable[[bool], int]) -> int:
    return p(True)


def second(p: Callable[[bool], int]) -> int:
    return p(False)


class FunctionalPairsTests(unittest.TestCase):
    def test_selector_laws(self) -> None:
        p = pair(17, -5)
        self.assertEqual(first(p), 17)
        self.assertEqual(second(p), -5)


if __name__ == "__main__":
    p = pair(4, 9)
    print(f"first={first(p)}, second={second(p)}")
