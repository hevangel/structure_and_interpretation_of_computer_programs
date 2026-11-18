from collections.abc import Callable, Iterator
from itertools import islice
import math
import unittest


def euler_stream(
    t: float, y: float, dt: float, derivative: Callable[[float, float], float]
) -> Iterator[tuple[float, float]]:
    while True:
        yield (t, y)
        # Explicit Euler: slope is evaluated at the current state.
        y += dt * derivative(t, y)
        t += dt


class StreamIntegrationTests(unittest.TestCase):
    def test_exponential_growth(self) -> None:
        samples = list(islice(euler_stream(0.0, 1.0, 0.001, lambda _t, y: y), 1001))
        time, value = samples[-1]
        self.assertLess(abs(time - 1.0), 1e-10)
        self.assertLess(abs(value - math.e), 0.002)


if __name__ == "__main__":
    time, value = list(islice(euler_stream(0.0, 1.0, 0.001, lambda _t, y: y), 1001))[-1]
    print(f"estimate e ≈ {value:.6f} at t = {time:.3f}")
