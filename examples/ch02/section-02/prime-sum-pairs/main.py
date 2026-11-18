import unittest


def is_prime(n: int) -> bool:
    if n < 2:
        return False
    divisor = 2
    while divisor * divisor <= n:
        if n % divisor == 0:
            return False
        divisor += 1
    return True


def pairs(limit: int) -> list[tuple[int, int]]:
    return [
        (smaller, larger)
        for larger in range(1, limit + 1)
        for smaller in range(1, larger)
        if is_prime(smaller + larger)
    ]


class PrimeSumPairsTests(unittest.TestCase):
    def test_filters_without_duplicates(self) -> None:
        result = pairs(6)
        self.assertEqual(len(result), 7)
        self.assertIn((1, 2), result)
        self.assertIn((5, 6), result)
        self.assertTrue(all(a < b and is_prime(a + b) for a, b in result))


if __name__ == "__main__":
    print(f"prime-sum pairs up to 6: {pairs(6)}")
