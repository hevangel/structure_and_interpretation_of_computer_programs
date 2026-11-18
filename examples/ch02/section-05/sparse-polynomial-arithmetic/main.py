from dataclasses import dataclass
import unittest


@dataclass(frozen=True)
class Polynomial:
    terms: dict[int, int]

    @classmethod
    def from_terms(cls, items: list[tuple[int, int]]) -> "Polynomial":
        terms: dict[int, int] = {}
        for degree, coefficient in items:
            terms[degree] = terms.get(degree, 0) + coefficient
        return cls({d: c for d, c in sorted(terms.items()) if c != 0})

    def add(self, other: "Polynomial") -> "Polynomial":
        merged = dict(self.terms)
        for degree, coeff in other.terms.items():
            merged[degree] = merged.get(degree, 0) + coeff
        return Polynomial({d: c for d, c in sorted(merged.items()) if c != 0})

    def multiply(self, other: "Polynomial") -> "Polynomial":
        prod: dict[int, int] = {}
        for a, x in self.terms.items():
            for b, y in other.terms.items():
                prod[a + b] = prod.get(a + b, 0) + x * y
        return Polynomial({d: c for d, c in sorted(prod.items()) if c != 0})

    def at(self, x: int) -> int:
        return sum(coeff * (x**degree) for degree, coeff in self.terms.items())


class SparsePolynomialArithmeticTests(unittest.TestCase):
    def test_polynomial_ring_operations(self) -> None:
        p = Polynomial.from_terms([(0, 1), (1, 1)])
        square = p.multiply(p)
        self.assertEqual(square, Polynomial.from_terms([(0, 1), (1, 2), (2, 1)]))
        self.assertEqual(square.at(3), 16)
        self.assertEqual(
            p.add(Polynomial.from_terms([(1, -1)])), Polynomial.from_terms([(0, 1)])
        )


if __name__ == "__main__":
    p = Polynomial.from_terms([(1, 1), (0, 1)])
    q = Polynomial.from_terms([(2, 1), (0, -1)])
    sq = p.multiply(p)
    print(f"(x+1)^2 = {sq}, p+q at 2 = {p.add(q).at(2)}")
