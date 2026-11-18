import unittest


def fibonacci(n: int, cache: dict[int, int]) -> int:
    if n < 2:
        return n
    if n in cache:
        return cache[n]
    # Each recursive dependency is evaluated at most once.
    answer = fibonacci(n - 1, cache) + fibonacci(n - 2, cache)
    cache[n] = answer
    return answer


class MemoizedFibonacciTests(unittest.TestCase):
    def test_cache_reuses_results(self) -> None:
        memo: dict[int, int] = {}
        self.assertEqual(fibonacci(10, memo), 55)
        entries = len(memo)
        self.assertEqual(fibonacci(10, memo), 55)
        self.assertEqual(entries, len(memo))
        self.assertEqual(fibonacci(80, memo), 23_416_728_348_467_685)


if __name__ == "__main__":
    cache: dict[int, int] = {}
    print(f"fib(80)={fibonacci(80, cache)} with {len(cache)} cache entries")
