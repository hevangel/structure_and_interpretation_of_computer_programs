from __future__ import annotations

import math
import unittest


def expmod(base: int, exp: int, m: int) -> int:
    assert m >= 2
    result = 1
    b = base % m
    e = exp
    while e > 0:
        if e & 1:
            result = (result * b) % m
        e >>= 1
        if e > 0:
            b = (b * b) % m
    return result


def all_coprime_fermat_pass(n: int) -> bool:
    """True iff a^n == a (mod n) holds for every 1 <= a < n coprime to n."""
    if n < 2:
        return False
    return all(expmod(a, n, n) == a for a in range(1, n) if math.gcd(a, n) == 1)


def _expmod_checked(base: int, exp: int, m: int) -> int:
    if exp == 0:
        return 1
    if exp % 2 == 0:
        root = _expmod_checked(base, exp // 2, m)
        if root == 0:
            return 0
        sq = (root * root) % m
        # Non-trivial square root of 1 mod m exposes a composite modulus.
        if root != 1 and root != m - 1 and sq == 1:
            return 0
        return sq
    sub = _expmod_checked(base, exp - 1, m)
    if sub == 0:
        return 0
    return (base * sub) % m


def miller_rabin_witness(n: int, base: int) -> bool:
    return n >= 2 and _expmod_checked(base, n - 1, n) == 1


def is_probable_prime_mr(n: int) -> bool:
    if n < 2:
        return False
    for p in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):
        if n == p:
            return True
        if n % p == 0:
            return False
        if p < n and not miller_rabin_witness(n, p):
            return False
    return True


def is_prime_trial(n: int) -> bool:
    if n < 2:
        return False
    d = 2
    while d * d <= n:
        if n % d == 0:
            return False
        d += 1
    return True


class FermatMillerRabinTest(unittest.TestCase):
    def test_carmichael_numbers(self) -> None:
        for c in (561, 1105, 1729, 2465, 2821, 6601):
            self.assertFalse(is_prime_trial(c))
            self.assertTrue(all_coprime_fermat_pass(c))
            self.assertFalse(is_probable_prime_mr(c))

    def test_miller_rabin_matches_trial_division(self) -> None:
        for n in range(10_000):
            self.assertEqual(is_probable_prime_mr(n), is_prime_trial(n))


if __name__ == "__main__":
    for c in (561, 1105, 1729, 2465, 2821, 6601):
        print(
            f"Carmichael {c}: fermat_all_coprime={all_coprime_fermat_pass(c)}, "
            f"mr_prime={is_probable_prime_mr(c)}"
        )
