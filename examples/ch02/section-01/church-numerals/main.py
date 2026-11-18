from collections.abc import Callable
import unittest


Church = Callable[[Callable[[int], int], int], int]


def zero() -> Church:
    return lambda _f, x: x


def succ(n: Church) -> Church:
    # The successor applies f once more than its predecessor.
    return lambda f, x: f(n(f, x))


def as_number(n: Church) -> int:
    return n(lambda x: x + 1, 0)


class ChurchNumeralsTests(unittest.TestCase):
    def test_function_is_number(self) -> None:
        three = succ(succ(succ(zero())))
        self.assertEqual(as_number(three), 3)
        self.assertEqual(three(lambda x: x * 2, 1), 8)


if __name__ == "__main__":
    two = succ(succ(zero()))
    print(f"church two = {as_number(two)}")
