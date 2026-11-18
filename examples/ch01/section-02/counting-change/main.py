from __future__ import annotations

import unittest


def count_change_tree(amount: int, coins: tuple[int, ...]) -> int:
    """Tree-recursive coin change counting combinations in O(branches) calls."""
    if amount == 0:
        return 1
    if amount < 0 or not coins:
        return 0
    return count_change_tree(amount, coins[1:]) + count_change_tree(amount - coins[0], coins)


def count_change_dp(target: int, coins: tuple[int, ...]) -> int:
    """Bottom-up dynamic programming coin-change count."""
    ways = [0] * (target + 1)
    ways[0] = 1
    for coin in coins:
        for amount in range(coin, target + 1):
            ways[amount] += ways[amount - coin]
    return ways[target]


def pascal(row: int, col: int) -> int:
    """0-indexed Pascal's triangle binomial coefficient."""
    if col == 0 or col == row:
        return 1
    if col < 0 or col > row:
        return 0
    return pascal(row - 1, col - 1) + pascal(row - 1, col)


def fib_log(n: int) -> int:
    """O(lg n) Fibonacci by squaring the transformation T_{p,q}."""
    a, b = 1, 0
    p, q = 0, 1
    count = n
    while count > 0:
        if count % 2 == 0:
            p, q = p * p + q * q, 2 * p * q + q * q
            count //= 2
        else:
            a, b = b * q + a * q + a * p, b * p + a * q
            count -= 1
    return b


def fib_iter(n: int) -> int:
    a, b = 0, 1
    for _ in range(n):
        a, b = b, a + b
    return a


class CountingChangeAndFibTest(unittest.TestCase):
    def test_tree_and_dp_agree(self) -> None:
        self.assertEqual(count_change_tree(100, (50, 25, 10, 5, 1)), 292)
        self.assertEqual(count_change_dp(100, (1, 5, 10, 25, 50)), 292)
        for amount in range(41):
            self.assertEqual(
                count_change_tree(amount, (25, 10, 5, 1)),
                count_change_dp(amount, (1, 5, 10, 25)),
            )

    def test_pascal_identities(self) -> None:
        for row in range(9):
            self.assertEqual(sum(pascal(row, col) for col in range(row + 1)), 1 << row)
        self.assertEqual(pascal(4, 2), 6)

    def test_fib_log_matches_iter(self) -> None:
        for n in range(91):
            self.assertEqual(fib_log(n), fib_iter(n))


if __name__ == "__main__":
    print(
        f"change(100): tree={count_change_tree(100, (50, 25, 10, 5, 1))}, "
        f"dp={count_change_dp(100, (1, 5, 10, 25, 50))}"
    )
    print(f"pascal(5, 2) = {pascal(5, 2)}, fib_log(50) = {fib_log(50)}")
