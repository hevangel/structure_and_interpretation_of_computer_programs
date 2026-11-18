import unittest


def mod_pow(base: int, exponent: int, modulus: int) -> int:
    assert modulus >= 2
    result = 1
    base %= modulus
    while exponent != 0:
        if exponent & 1:
            result = (result * base) % modulus
        exponent >>= 1
        if exponent > 0:
            base = (base * base) % modulus
    return result


def passes_fermat(n: int, base: int) -> bool:
    return n >= 2 and mod_pow(base, n, n) == base % n


def trial_prime(n: int) -> bool:
    if n < 2:
        return False
    divisor = 2
    while divisor * divisor <= n:
        if n % divisor == 0:
            return False
        divisor += 1
    return True


class FermatTestTests(unittest.TestCase):
    def test_prime_test_is_not_proof(self) -> None:
        self.assertEqual(mod_pow(2, 10, 1000), 24)
        self.assertTrue(passes_fermat(13, 2))
        self.assertTrue(passes_fermat(561, 2))
        self.assertFalse(trial_prime(561))
        self.assertTrue(trial_prime(563))


if __name__ == "__main__":
    print(f"561 passes base-2 Fermat? {passes_fermat(561, 2)}")
    print(f"561 is actually prime? {trial_prime(561)}")
