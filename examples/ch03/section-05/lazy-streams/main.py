from collections.abc import Iterator
from itertools import count, islice
import unittest


def is_prime(n: int) -> bool:
    if n < 2:
        return False
    d = 2
    while d * d <= n:
        if n % d == 0:
            return False
        d += 1
    return True


def first_primes(n: int) -> list[int]:
    # Consumers decide how many elements of the unbounded stream exist.
    stream: Iterator[int] = (k for k in count(2) if is_prime(k))
    return list(islice(stream, n))


class LazyStreamsTests(unittest.TestCase):
    def test_finite_prefix(self) -> None:
        self.assertEqual(first_primes(7), [2, 3, 5, 7, 11, 13, 17])
        self.assertEqual(first_primes(0), [])


if __name__ == "__main__":
    print(f"first ten primes: {first_primes(10)}")
