import unittest


def power(base: int, exponent: int) -> int:
    result = 1
    while exponent > 0:
        # Odd exponent folds one factor of base into the running product.
        if exponent % 2 == 1:
            result *= base
        base *= base
        exponent //= 2
    return result


class FastExponentiationTests(unittest.TestCase):
    def test_powers(self) -> None:
        self.assertEqual(power(3, 13), 1_594_323)
        self.assertEqual(power(5, 0), 1)
        self.assertEqual(power(-2, 5), -32)


if __name__ == "__main__":
    print(f"3^13 = {power(3, 13)}")
