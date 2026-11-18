from collections.abc import Callable
import unittest


def accumulate(
    start: int,
    end: int,
    identity: int,
    term: Callable[[int], int],
    combine: Callable[[int, int], int],
) -> int:
    result = identity
    for x in range(start, end + 1):
        result = combine(result, term(x))
    return result


class AccumulateTests(unittest.TestCase):
    def test_two_combiners(self) -> None:
        self.assertEqual(accumulate(1, 5, 0, lambda x: x, lambda a, b: a + b), 15)
        self.assertEqual(accumulate(1, 4, 1, lambda x: x, lambda a, b: a * b), 24)


if __name__ == "__main__":
    sum_of_squares = accumulate(1, 5, 0, lambda x: x * x, lambda a, b: a + b)
    print(f"sum of squares 1..5 = {sum_of_squares}")
