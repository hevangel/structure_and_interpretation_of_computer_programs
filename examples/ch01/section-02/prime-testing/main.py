import unittest


def is_prime(n: int) -> bool:
    if n < 2:
        return False
    if n == 2:
        return True
    if n % 2 == 0:
        return False
    d = 3
    # If n is composite, it has a factor <= isqrt(n).
    while d * d <= n:
        if n % d == 0:
            return False
        d += 2
    return True


class PrimeTestingTests(unittest.TestCase):
    def test_known_values(self) -> None:
        self.assertFalse(is_prime(1))
        self.assertTrue(is_prime(2) and is_prime(97))
        self.assertFalse(is_prime(9) or is_prime(221))


if __name__ == "__main__":
    print(f"101 prime? {is_prime(101)}")
