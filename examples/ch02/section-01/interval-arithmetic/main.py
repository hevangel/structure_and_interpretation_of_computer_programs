from __future__ import annotations

from dataclasses import dataclass
import unittest


@dataclass(frozen=True)
class Interval:
    low: float
    high: float

    def __post_init__(self) -> None:
        assert self.low <= self.high

    @classmethod
    def make_center_percent(cls, center: float, percent_tol: float) -> Interval:
        assert percent_tol >= 0.0
        width = abs(center) * (percent_tol / 100.0)
        return cls(center - width, center + width)

    def center(self) -> float:
        return 0.5 * (self.low + self.high)

    def width(self) -> float:
        return 0.5 * (self.high - self.low)

    def add(self, other: Interval) -> Interval:
        return Interval(self.low + other.low, self.high + other.high)

    def sub(self, other: Interval) -> Interval:
        return Interval(self.low - other.high, self.high - other.low)

    def mul(self, other: Interval) -> Interval:
        products = (
            self.low * other.low,
            self.low * other.high,
            self.high * other.low,
            self.high * other.high,
        )
        return Interval(min(products), max(products))

    def mul_fast(self, other: Interval) -> Interval:
        a, b = self.low, self.high
        c, d = other.low, other.high
        if a >= 0.0:
            if c >= 0.0:
                return Interval(a * c, b * d)
            if d <= 0.0:
                return Interval(b * c, a * d)
            return Interval(b * c, b * d)
        if b <= 0.0:
            if c >= 0.0:
                return Interval(a * d, b * c)
            if d <= 0.0:
                return Interval(b * d, a * c)
            return Interval(a * d, a * c)
        if c >= 0.0:
            return Interval(a * d, b * d)
        if d <= 0.0:
            return Interval(b * c, a * c)
        return Interval(min(a * d, b * c), max(a * c, b * d))

    def reciprocal(self) -> Interval:
        if self.low <= 0.0 <= self.high:
            raise ZeroDivisionError("cannot divide by an interval spanning zero")
        return Interval(1.0 / self.high, 1.0 / self.low)

    def div(self, other: Interval) -> Interval:
        return self.mul(other.reciprocal())


def par1(r1: Interval, r2: Interval) -> Interval:
    return r1.mul(r2).div(r1.add(r2))


def par2(r1: Interval, r2: Interval) -> Interval:
    one = Interval(1.0, 1.0)
    return one.div(one.div(r1).add(one.div(r2)))


class IntervalArithmeticTest(unittest.TestCase):
    def test_mul_fast_matches_mul(self) -> None:
        samples = [
            Interval(2.0, 4.0),
            Interval(-5.0, -1.0),
            Interval(-3.0, 6.0),
            Interval(0.0, 3.0),
            Interval(-4.0, 0.0),
        ]
        for a in samples:
            for b in samples:
                self.assertEqual(a.mul_fast(b), a.mul(b))

    def test_par2_tighter_than_par1(self) -> None:
        r1 = Interval.make_center_percent(100.0, 5.0)
        r2 = Interval.make_center_percent(200.0, 5.0)
        p1, p2 = par1(r1, r2), par2(r1, r2)
        self.assertLess(p2.width(), p1.width())
        self.assertLessEqual(p1.low, p2.low)
        self.assertLessEqual(p2.high, p1.high)

    def test_zero_straddling_div_raises(self) -> None:
        with self.assertRaises(ZeroDivisionError):
            Interval(1.0, 2.0).div(Interval(-1.0, 1.0))


if __name__ == "__main__":
    r1 = Interval.make_center_percent(100.0, 5.0)
    r2 = Interval.make_center_percent(200.0, 2.0)
    p1, p2 = par1(r1, r2), par2(r1, r2)
    print(f"par1 width={p1.width():.4f}, par2 width={p2.width():.4f}")
