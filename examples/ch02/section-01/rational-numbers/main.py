from dataclasses import dataclass
import math
import unittest


@dataclass(frozen=True)
class Rational:
    num: int
    den: int

    @classmethod
    def make(cls, num: int, den: int) -> "Rational":
        assert den != 0
        sign = -1 if den < 0 else 1
        g = math.gcd(abs(num), abs(den))
        return cls(sign * num // g, sign * den // g)

    def add(self, other: "Rational") -> "Rational":
        return Rational.make(
            self.num * other.den + other.num * self.den, self.den * other.den
        )


class RationalNumbersTests(unittest.TestCase):
    def test_canonical_representation(self) -> None:
        self.assertEqual(Rational.make(2, -4), Rational.make(-1, 2))
        self.assertEqual(
            Rational.make(1, 2).add(Rational.make(1, 3)), Rational.make(5, 6)
        )


if __name__ == "__main__":
    print(Rational.make(1, 2).add(Rational.make(1, 3)))
