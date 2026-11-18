import unittest


def recursive(n: int) -> int:
    if n < 2:
        return n
    return recursive(n - 1) + recursive(n - 2)


def iterative(n: int) -> int:
    prev, curr = 0, 1
    for _ in range(n):
        prev, curr = curr, prev + curr
    return prev


class FibonacciProcessesTests(unittest.TestCase):
    def test_same_sequence(self) -> None:
        for n in range(26):
            self.assertEqual(recursive(n), iterative(n))
        self.assertEqual(iterative(10), 55)


if __name__ == "__main__":
    print(f"fib(20): rec={recursive(20)}, iter={iterative(20)}")
