# 2.5 Systems with Generic Operations · [Chapter 2](../README.md)

## The big idea

Generic arithmetic combines distinct types—integers, rationals, reals, complex numbers, and even **symbolic polynomials**—under a single `+`, `-`, `*`, `/` interface.

To add `Int(3) + Rat(1, 2)` or simplify `Complex(Rat(6, 1), 0)` back down to `Int(6)`, a **numeric tower** pairs upward **coercion (`raise`)** with lossless downward **simplification (`drop` via `project`)**. Even polynomials with rational or polynomial coefficients reuse those exact same generic `+` and `*` operations!

## What you should focus on

Focus on ****generic operator dispatch, ordered coercion towers (`Int < Rat < Real < Complex`), `raise` vs `drop`, and sparse symbolic polynomial algebra & GCD****. Across every example, test which callers remain completely untouched when you swap the internal representation.

## Connecting the concept to Rust and Python

- **In Rust:** Traits (`Add`, `Mul`, custom `TowerValue`) give static polymorphism for homogeneous types, while heterogeneous mixed-type expressions (`Int + Complex`) use a tagged enum or dynamic registry with explicit `raise`/`project` transitions.
- **In Python:** Python's numeric tower and dunder hooks (`__add__`, `__radd__`) natively demonstrate coercion (`int -> float -> complex`), and `Fraction`/polynomial classes layer generic symbolic algebra cleanly on top.
- **Key contrast:** Notice how both `numeric-tower-coercion` and `symbolic-polynomials` mirror each other: avoiding ad-hoc `N^2` pairwise converters by ordering types into a clean `Int -> Rat -> Real -> Complex` chain.

## Subsections

- [2.5.1 Generic Arithmetic](subsection-01/INTRO.md)
- [2.5.2 Combining Different Types](subsection-02/INTRO.md)
- [2.5.3 Symbolic Algebra](subsection-03/INTRO.md)

## Runnable examples in this section

- [2.5 Generic arithmetic with a numeric tower](../../../examples/ch02/section-05/generic-arithmetic/README.md)
- [2.5 Sparse polynomial addition and multiplication](../../../examples/ch02/section-05/sparse-polynomial-arithmetic/README.md)
- [2.5 Four-level numeric tower, coercion, raise, and drop](../../../examples/ch02/section-05/numeric-tower-coercion/README.md)
- [2.5 Symbolic polynomial algebra, pseudo-remainder GCD, and rational functions](../../../examples/ch02/section-05/symbolic-polynomials/README.md)

## Check your understanding

1. Why is `drop` safe only when `raise(project(x)) == x` holds cleanly?
2. Why does integer polynomial GCD require **pseudo-remainder** scaling `(leading_coeff)^(deg_diff + 1)` before dividing sparse term lists?

**Experiment:** Compute `gcd_poly` on two multi-term polynomials with fractional intermediate steps in `symbolic-polynomials` and verify integer coefficient preservation.

[Chapter 2 overview](../README.md) · [Edition notes](../../../docs/editions.md)
