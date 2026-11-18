from __future__ import annotations

from dataclasses import dataclass
import math
import unittest


@dataclass(frozen=True)
class Poly:
    var: str
    terms: dict[int, int]

    @classmethod
    def from_terms(cls, var: str, items: list[tuple[int, int]]) -> Poly:
        terms: dict[int, int] = {}
        for deg, coeff in items:
            terms[deg] = terms.get(deg, 0) + coeff
        return cls(var, {d: c for d, c in sorted(terms.items()) if c != 0})

    def is_zero(self) -> bool:
        return not self.terms

    def order(self) -> int:
        return max(self.terms, default=0)

    def leading_coeff(self) -> int:
        return self.terms[self.order()] if self.terms else 0

    def negate(self) -> Poly:
        return Poly(self.var, {d: -c for d, c in self.terms.items()})

    def scale(self, factor: int) -> Poly:
        if factor == 0:
            return Poly(self.var, {})
        return Poly(self.var, {d: c * factor for d, c in self.terms.items()})

    def add(self, other: Poly) -> Poly:
        assert self.var == other.var
        merged = dict(self.terms)
        for d, c in other.terms.items():
            merged[d] = merged.get(d, 0) + c
        return Poly(self.var, {d: c for d, c in sorted(merged.items()) if c != 0})

    def sub(self, other: Poly) -> Poly:
        return self.add(other.negate())

    def mul(self, other: Poly) -> Poly:
        assert self.var == other.var
        prod: dict[int, int] = {}
        for d1, c1 in self.terms.items():
            for d2, c2 in other.terms.items():
                prod[d1 + d2] = prod.get(d1 + d2, 0) + c1 * c2
        return Poly(self.var, {d: c for d, c in sorted(prod.items()) if c != 0})

    def div_rem(self, divisor: Poly) -> tuple[Poly, Poly]:
        assert self.var == divisor.var and not divisor.is_zero()
        q = Poly(self.var, {})
        r = self
        d_ord, d_lc = divisor.order(), divisor.leading_coeff()
        while not r.is_zero() and r.order() >= d_ord:
            r_lc = r.leading_coeff()
            if r_lc % d_lc != 0:
                break
            term = Poly.from_terms(self.var, [(r.order() - d_ord, r_lc // d_lc)])
            q = q.add(term)
            r = r.sub(term.mul(divisor))
        return q, r

    def pseudoremainder(self, divisor: Poly) -> Poly:
        o1, o2 = self.order(), divisor.order()
        if o1 < o2:
            return self
        factor = divisor.leading_coeff() ** (1 + o1 - o2)
        _, rem = self.scale(factor).div_rem(divisor)
        return rem

    def content_gcd(self) -> Poly:
        if self.is_zero():
            return self
        g = math.gcd(*self.terms.values())
        sign = -1 if self.leading_coeff() < 0 else 1
        div = g * sign
        return Poly(self.var, {d: c // div for d, c in self.terms.items()})

    def gcd(self, other: Poly) -> Poly:
        assert self.var == other.var
        a, b = self, other
        while not b.is_zero():
            a, b = b, a.pseudoremainder(b)
        return a.content_gcd()


def reduce_poly_fraction(num: Poly, den: Poly) -> tuple[Poly, Poly]:
    assert not den.is_zero()
    g = num.gcd(den)
    factor = g.leading_coeff() ** (1 + max(num.order(), den.order()) - g.order())
    n_red, rem_n = num.scale(factor).div_rem(g)
    d_red, rem_d = den.scale(factor).div_rem(g)
    assert rem_n.is_zero() and rem_d.is_zero()
    common = math.gcd(*n_red.terms.values(), *d_red.terms.values())
    sign = -1 if d_red.leading_coeff() < 0 else 1
    div = common * sign
    return (
        Poly(n_red.var, {d: c // div for d, c in n_red.terms.items()}),
        Poly(d_red.var, {d: c // div for d, c in d_red.terms.items()}),
    )


class SymbolicPolynomialsTest(unittest.TestCase):
    def test_division_and_gcd(self) -> None:
        x2_m_1 = Poly.from_terms("x", [(2, 1), (0, -1)])
        x_m_1 = Poly.from_terms("x", [(1, 1), (0, -1)])
        x_p_1 = Poly.from_terms("x", [(1, 1), (0, 1)])
        q, r = x2_m_1.div_rem(x_m_1)
        self.assertEqual(q, x_p_1)
        self.assertTrue(r.is_zero())

    def test_fraction_reduction(self) -> None:
        p1 = Poly.from_terms("x", [(1, 1), (0, 1)])
        p2 = Poly.from_terms("x", [(1, 1), (0, -1)])
        q = Poly.from_terms("x", [(1, 3), (0, -2)])
        rn, rd = reduce_poly_fraction(p1.mul(q), p2.mul(q))
        self.assertEqual((rn, rd), (p1, p2))


if __name__ == "__main__":
    p1 = Poly.from_terms("x", [(2, 1), (0, -1)])
    p2 = Poly.from_terms("x", [(1, 1), (0, -1)])
    q, r = p1.div_rem(p2)
    print("(x^2 - 1)/(x - 1) =", q.terms, "rem =", r.terms)
