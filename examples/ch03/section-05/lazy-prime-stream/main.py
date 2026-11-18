from collections.abc import Iterator
from itertools import islice, takewhile
import unittest


def primes_stream() -> Iterator[int]:
    found: list[int] = []
    candidate = 2
    while True:
        # Composite n always has some prime divisor <= isqrt(n).
        if all(candidate % p != 0 for p in takewhile(lambda p: p * p <= candidate, found)):
            found.append(candidate)
            yield candidate
        candidate += 1


class LazyPrimeStreamTests(unittest.TestCase):
    def test_prefix_matches_known_primes(self) -> None:
        self.assertEqual(
            list(islice(primes_stream(), 10)),
            [2, 3, 5, 7, 11, 13, 17, 19, 23, 29],
        )
        self.assertEqual(list(islice(primes_stream(), 0)), [])


if __name__ == "__main__":
    print(f"first 10 primes: {list(islice(primes_stream(), 10))}")
