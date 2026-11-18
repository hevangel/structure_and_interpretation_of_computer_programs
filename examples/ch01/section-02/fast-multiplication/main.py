import unittest


def multiply(a: int, b: int) -> int:
    result = 0
    while b != 0:
        if b & 1:
            result += a
        b >>= 1
        if b != 0:
            a += a
    return result


class FastMultiplicationTests(unittest.TestCase):
    def test_repeated_doubling(self) -> None:
        for a, b in ((0, 100), (7, 0), (37, 42), (1, 100_000), (1000, 500)):
            self.assertEqual(multiply(a, b), a * b)
        self.assertEqual(multiply(1, 1 << 63), 1 << 63)


if __name__ == "__main__":
    print(f"37 * 42 = {multiply(37, 42)}")
