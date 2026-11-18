import math
import unittest


def cube_root(x: float) -> float:
    assert math.isfinite(x), "input must be finite"
    if x == 0.0:
        return 0.0
    magnitude = abs(x)
    guess = max(magnitude, 1.0)
    for _ in range(250):
        # Newton's method applied to f(g) = g^3 - |x|.
        nxt = (2.0 * guess + magnitude / (guess * guess)) / 3.0
        if abs(nxt - guess) <= 1e-14 * max(nxt, 1.0):
            guess = nxt
            break
        guess = nxt
    return math.copysign(guess, x)


class CubeRootTests(unittest.TestCase):
    def test_negative_and_positive_inputs(self) -> None:
        for x in (0.0, 1.0, 8.0, -64.0, 125.0, 1e-9, 1e12):
            result = cube_root(x)
            ref = math.copysign(abs(x) ** (1.0 / 3.0), x) if x != 0.0 else 0.0
            self.assertLessEqual(abs(result - ref), 1e-9 * max(abs(ref), 1.0))


if __name__ == "__main__":
    print(f"cube root of 27 is {cube_root(27.0):.9f}")
