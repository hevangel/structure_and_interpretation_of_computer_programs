import unittest


def recursive(n: int) -> int:
    return 1 if n == 0 else n * recursive(n - 1)


def iterative(n: int) -> int:
    result = 1
    # Invariant before iteration i: result = (i - 1)!.
    for i in range(1, n + 1):
        result *= i
    return result


class FactorialProcessesTests(unittest.TestCase):
    def test_both_processes_agree(self) -> None:
        for n in range(21):
            self.assertEqual(recursive(n), iterative(n))
        self.assertEqual(iterative(5), 120)


if __name__ == "__main__":
    print(f"10! (rec={recursive(10)}, iter={iterative(10)})")
