import unittest


def horner(coefficients: list[float], x: float) -> float:
    # For [c0, c1, ..., cn], fold cn*x^n + ... + c0 from highest degree down.
    acc = 0.0
    for coefficient in reversed(coefficients):
        acc = acc * x + coefficient
    return acc


class HornerEvaluationTests(unittest.TestCase):
    def test_polynomial_values(self) -> None:
        self.assertEqual(horner([1.0, 0.0, 3.0, 2.0], 2.0), 29.0)
        self.assertEqual(horner([7.0], 100.0), 7.0)
        self.assertEqual(horner([], 3.0), 0.0)


if __name__ == "__main__":
    print(f"p(2) = {horner([1.0, 0.0, 3.0, 2.0], 2.0)}")
