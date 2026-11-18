# 2.3.2 Symbolic Differentiation

## What this subsection teaches

Symbolic differentiation applies exact algebraic rewrite rules directly to an AST: `d(u + v)/dx = du/dx + dv/dx`, `d(u * v)/dx = u * dv/dx + v * du/dx`, and `d(u^n)/dx = n * u^(n - 1) * du/dx`—plus algebraic simplification (`x * 0 -> 0`, `x + 0 -> x`, `x^1 -> x`) to prevent trees from exploding exponentially.

## Connecting the concept to Rust and Python

- **In Rust:** Exhaustive `match` on `enum Expr` makes rule completeness checkable at compile time, while `Box<Expr>` keeps recursive trees compact.
- **In Python:** Python uses immutable `@dataclass(frozen=True)` AST nodes (`Add`, `Mul`, `Pow`, `Var`, `Const`) with `isinstance` or `match/case` simplification rules.
- **Why compare both:** Compare [`symbolic-differentiation`](../../../../examples/ch02/section-03/symbolic-differentiation/README.md) with [`ordered-sets-infix-deriv`](../../../../examples/ch02/section-03/ordered-sets-infix-deriv/README.md) (which parses infix expressions and handles exponent rules cleanly).

## Exercise

Differentiate `(x + 1)^3` symbolically and verify simplification reduces trivial `+ 0` and `* 1` terms.

[Parent section](../INTRO.md) · [Chapter 2 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
