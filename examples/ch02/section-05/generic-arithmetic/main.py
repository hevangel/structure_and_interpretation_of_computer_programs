from dataclasses import dataclass
import math
import unittest


@dataclass(frozen=True)
class IntNum:
    value: int

    def as_real(self) -> float:
        return float(self.value)

    def as_rat(self) -> tuple[int, int]:
        return (self.value, 1)


@dataclass(frozen=True)
class RatNum:
    num: int
    den: int

    def as_real(self) -> float:
        return self.num / self.den

    def as_rat(self) -> tuple[int, int]:
        return (self.num, self.den)


@dataclass(frozen=True)
class RealNum:
    value: float

    def as_real(self) -> float:
        return self.value


Number = IntNum | RatNum | RealNum


def make_rat(n: int, d: int) -> RatNum:
    assert d != 0
    sign = -1 if d < 0 else 1
    g = math.gcd(abs(n), abs(d))
    return RatNum(sign * n // g, sign * d // g)


def add(a: Number, b: Number) -> Number:
    if isinstance(a, RealNum) or isinstance(b, RealNum):
        return RealNum(a.as_real() + b.as_real())
    if isinstance(a, IntNum) and isinstance(b, IntNum):
        return IntNum(a.value + b.value)
    an, ad = a.as_rat()
    bn, bd = b.as_rat()
    return make_rat(an * bd + bn * ad, ad * bd)


class GenericArithmeticTests(unittest.TestCase):
    def test_mixed_arithmetic(self) -> None:
        self.assertEqual(add(IntNum(2), make_rat(1, 3)), make_rat(7, 3))
        self.assertEqual(add(IntNum(2), RealNum(0.5)), RealNum(2.5))


if __name__ == "__main__":
    print(add(IntNum(2), make_rat(1, 3)))
