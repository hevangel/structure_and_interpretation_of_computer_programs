from dataclasses import dataclass
import math
import unittest


class Complex:
    def re(self) -> float:
        raise NotImplementedError

    def im(self) -> float:
        raise NotImplementedError

    def radius(self) -> float:
        return math.hypot(self.re(), self.im())

    def angle(self) -> float:
        return math.atan2(self.im(), self.re())

    def plus(self, other: "Complex") -> "Cartesian":
        return Cartesian(self.re() + other.re(), self.im() + other.im())

    def times(self, other: "Complex") -> "Polar":
        return Polar(self.radius() * other.radius(), self.angle() + other.angle())


@dataclass(frozen=True)
class Cartesian(Complex):
    _re: float
    _im: float

    def re(self) -> float:
        return self._re

    def im(self) -> float:
        return self._im


@dataclass(frozen=True)
class Polar(Complex):
    _r: float
    _angle: float

    def re(self) -> float:
        return self._r * math.cos(self._angle)

    def im(self) -> float:
        return self._r * math.sin(self._angle)


class ComplexRepresentationsTests(unittest.TestCase):
    def test_two_operations(self) -> None:
        a = Cartesian(1.0, 1.0)
        b = Cartesian(1.0, -1.0)
        self.assertLess(abs(a.times(b).re() - 2.0), 1e-12)
        self.assertLess(abs(a.plus(b).im()), 1e-12)


if __name__ == "__main__":
    a = Cartesian(1.0, 1.0)
    b = Cartesian(1.0, -1.0)
    print(f"(1+i)+(1-i) = {a.plus(b).re()} + {a.plus(b).im()}i")
    print(f"(1+i)(1-i) = {a.times(b).re()} + {a.times(b).im()}i")
