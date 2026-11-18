import unittest


def gcd(a: int, b: int) -> int:
    while b != 0:
        # Divisors common to (a, b) are exactly those common to (b, a % b).
        a, b = b, a % b
    return a


class EuclideanGcdTests(unittest.TestCase):
    def test_common_divisors(self) -> None:
        self.assertEqual(gcd(206, 40), 2)
        self.assertEqual(gcd(0, 9), 9)
        self.assertEqual(gcd(13, 13), 13)


if __name__ == "__main__":
    print(f"gcd(206, 40) = {gcd(206, 40)}")
