from collections.abc import Callable
import unittest


def simpson(f: Callable[[float], float], a: float, b: float, n: int) -> float:
    assert n > 0 and n % 2 == 0, "number of subintervals must be positive and even"
    h = (b - a) / n
    total = f(a) + f(b)
    for k in range(1, n):
        weight = 2.0 if k % 2 == 0 else 4.0
        total += weight * f(a + k * h)
    return total * h / 3.0


class SimpsonIntegrationTests(unittest.TestCase):
    def test_cubic_exactness(self) -> None:
        self.assertLess(abs(simpson(lambda x: x**3, 0.0, 1.0, 10) - 0.25), 1e-12)


if __name__ == "__main__":
    print(f"integral x^3 from 0 to 1 ≈ {simpson(lambda x: x * x * x, 0.0, 1.0, 100)}")
