# 2.5.3 Symbolic Algebra

## What this subsection teaches

Extending generic operations to **polynomials** (`3*x^4 + 2*x + 5`) shows the real payoff of symbolic data + generic `+`/`*`: coefficients themselves can be integers, rationals, or nested polynomials in another variable!

Dense arrays (`[5, 2, 0, 0, 3]`) waste memory on `x^1000 + 1`, so **sparse term lists** store only nonzero `(degree, coeff)` pairs in descending order. Computing exact polynomial **GCD** uses **pseudo-remainders**—multiplying by `lead(divisor)^(deg_diff + 1)` before subtracting so integer coefficients never introduce fractions—and produces simplified **rational functions**.

## Connecting the concept to Rust and Python

- **In Rust:** Rust represents sparse polynomials cleanly via `BTreeMap<usize, Coeff>` or sorted `Vec<(usize, i64)>`, keeping terms canonical (nonzero coefficients, sorted degrees) and returning `Result` on zero divisors.
- **In Python:** Python uses `dict[int, int]` or sorted term tuples cleanly with arbitrary-precision integer coefficients (crucial when pseudo-remainder GCD exponents grow large!).
- **Why compare both:** Compare [`sparse-polynomial-arithmetic`](../../../../examples/ch02/section-05/sparse-polynomial-arithmetic/README.md) and [`symbolic-polynomials`](../../../../examples/ch02/section-05/symbolic-polynomials/README.md) to see why arbitrary-precision Python integers shine on high-degree pseudo-remainders.

## Exercise

Divide `x^5 - 1` by `x^2 - 1` using sparse polynomial long division and verify `(q, r)` satisfies `q * d + r == dividend`.

[Parent section](../INTRO.md) · [Chapter 2 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
