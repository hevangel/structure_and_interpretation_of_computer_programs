from __future__ import annotations

from dataclasses import dataclass
from fractions import Fraction
import math
import unittest


@dataclass(frozen=True)
class TowerInt:
    value: int

    rank = 0

    def raise_once(self) -> TowerRat:
        return TowerRat(Fraction(self.value, 1))

    def project(self) -> None:
        return None


@dataclass(frozen=True)
class TowerRat:
    value: Fraction

    rank = 1

    def raise_once(self) -> TowerReal:
        return TowerReal(float(self.value))

    def project(self) -> TowerInt:
        return TowerInt(self.value.numerator // self.value.denominator)


@dataclass(frozen=True)
class TowerReal:
    value: float

    rank = 2

    def raise_once(self) -> TowerComplex:
        return TowerComplex(complex(self.value, 0.0))

    def project(self) -> TowerRat:
        return TowerRat(Fraction(round(self.value * 1_000_000), 1_000_000))


@dataclass(frozen=True)
class TowerComplex:
    value: complex

    rank = 3

    def raise_once(self) -> None:
        return None

    def project(self) -> TowerReal:
        return TowerReal(self.value.real)


TowerNum = TowerInt | TowerRat | TowerReal | TowerComplex


def raise_to(x: TowerNum, target_rank: int) -> TowerNum:
    cur: TowerNum = x
    while cur.rank < target_rank:
        nxt = cur.raise_once()
        assert nxt is not None
        cur = nxt
    return cur


def equ(a: TowerNum, b: TowerNum) -> bool:
    top = max(a.rank, b.rank)
    x, y = raise_to(a, top), raise_to(b, top)
    if isinstance(x, TowerInt) and isinstance(y, TowerInt):
        return x.value == y.value
    if isinstance(x, TowerRat) and isinstance(y, TowerRat):
        return x.value == y.value
    if isinstance(x, TowerReal) and isinstance(y, TowerReal):
        return math.isclose(x.value, y.value, abs_tol=1e-12)
    if isinstance(x, TowerComplex) and isinstance(y, TowerComplex):
        return abs(x.value - y.value) < 1e-12
    raise AssertionError("unreachable")


def is_zero(x: TowerNum) -> bool:
    return equ(x, TowerInt(0))


def drop(x: TowerNum) -> TowerNum:
    cur: TowerNum = x
    while True:
        lower = cur.project()
        if lower is None:
            return cur
        re_raised = lower.raise_once()
        if re_raised is not None and equ(re_raised, cur):
            cur = lower
        else:
            return cur


def tower_add(a: TowerNum, b: TowerNum) -> TowerNum:
    top = max(a.rank, b.rank)
    x, y = raise_to(a, top), raise_to(b, top)
    if isinstance(x, TowerInt) and isinstance(y, TowerInt):
        raw: TowerNum = TowerInt(x.value + y.value)
    elif isinstance(x, TowerRat) and isinstance(y, TowerRat):
        raw = TowerRat(x.value + y.value)
    elif isinstance(x, TowerReal) and isinstance(y, TowerReal):
        raw = TowerReal(x.value + y.value)
    elif isinstance(x, TowerComplex) and isinstance(y, TowerComplex):
        raw = TowerComplex(x.value + y.value)
    else:
        raise AssertionError("unreachable")
    return drop(raw)


def tower_mul(a: TowerNum, b: TowerNum) -> TowerNum:
    top = max(a.rank, b.rank)
    x, y = raise_to(a, top), raise_to(b, top)
    if isinstance(x, TowerInt) and isinstance(y, TowerInt):
        raw: TowerNum = TowerInt(x.value * y.value)
    elif isinstance(x, TowerRat) and isinstance(y, TowerRat):
        raw = TowerRat(x.value * y.value)
    elif isinstance(x, TowerReal) and isinstance(y, TowerReal):
        raw = TowerReal(x.value * y.value)
    elif isinstance(x, TowerComplex) and isinstance(y, TowerComplex):
        raw = TowerComplex(x.value * y.value)
    else:
        raise AssertionError("unreachable")
    return drop(raw)


class NumericTowerTest(unittest.TestCase):
    def test_multi_step_raise_and_add(self) -> None:
        self.assertEqual(tower_add(TowerInt(5), TowerComplex(1.5 - 2j)), TowerComplex(6.5 - 2j))
        self.assertEqual(tower_add(TowerInt(2), TowerRat(Fraction(1, 2))), TowerRat(Fraction(5, 2)))

    def test_drop_simplifies(self) -> None:
        self.assertEqual(drop(TowerComplex(6 + 0j)), TowerInt(6))
        prod = tower_mul(TowerComplex(3 + 4j), TowerComplex(3 - 4j))
        self.assertEqual(prod, TowerInt(25))
        self.assertTrue(is_zero(TowerComplex(0j)))


if __name__ == "__main__":
    print("2 + (3+4j) =", tower_add(TowerInt(2), TowerComplex(3 + 4j)))
    print("(3+4j)*(3-4j) dropped =", tower_mul(TowerComplex(3 + 4j), TowerComplex(3 - 4j)))
