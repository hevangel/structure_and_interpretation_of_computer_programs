from typing import Any
import unittest


Expr = int | str | tuple[str, Any, Any]


def add(a: Expr, b: Expr) -> Expr:
    if a == 0:
        return b
    if b == 0:
        return a
    return ("+", a, b)


def mul(a: Expr, b: Expr) -> Expr:
    if a == 0 or b == 0:
        return 0
    if a == 1:
        return b
    if b == 1:
        return a
    return ("*", a, b)


def derivative(e: Expr) -> Expr:
    if isinstance(e, int):
        return 0
    if e == "x":
        return 1
    assert isinstance(e, tuple)
    op, a, b = e
    if op == "+":
        return add(derivative(a), derivative(b))
    if op == "*":
        return add(mul(derivative(a), b), mul(a, derivative(b)))
    raise ValueError(f"unsupported operator {op}")


def evaluate(e: Expr, x: int) -> int:
    if isinstance(e, int):
        return e
    if e == "x":
        return x
    assert isinstance(e, tuple)
    op, a, b = e
    return evaluate(a, x) + evaluate(b, x) if op == "+" else evaluate(a, x) * evaluate(b, x)


class SymbolicDifferentiationTests(unittest.TestCase):
    def test_product_rule(self) -> None:
        e = mul("x", "x")
        self.assertEqual(evaluate(derivative(e), 7), 14)
        self.assertEqual(derivative(5), 0)


if __name__ == "__main__":
    x_squared = mul("x", "x")
    print(f"d(x*x)/dx at 3 = {evaluate(derivative(x_squared), 3)}")
