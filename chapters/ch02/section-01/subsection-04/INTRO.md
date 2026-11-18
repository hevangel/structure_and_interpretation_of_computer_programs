# 2.1.4 Interval Arithmetic

## What this subsection teaches

Physical measurements carry tolerances `[lo, hi]` (`100 ± 5%`). Adding `[a, b] + [c, d] = [a+c, b+d]` and multiplying across extreme corners preserves guaranteed bounds.

The trap is **variable correlation (the dependency problem)**: algebraically equivalent formulas `R1 * R2 / (R1 + R2)` (`par1`) and `1 / (1/R1 + 1/R2)` (`par2`) yield *different* widths because interval arithmetic treats repeated `R1` occurrences as independent uncertain resistors!

## Connecting the concept to Rust and Python

- **In Rust:** A `struct Interval { lo: f64, hi: f64 }` validates `lo <= hi` and checks `0.0` containment before reciprocal division via `Result<Interval, &str>`.
- **In Python:** A `@dataclass(frozen=True)` raises `ValueError` on division by intervals straddling zero and supports operator overloads or `add`/`mul`/`div` helpers cleanly.
- **Why compare both:** Because `par2` uses each uncertain interval `R1`, `R2` **only once**, both Rust and Python demonstrate tighter bounds in [`interval-arithmetic`](../../../../examples/ch02/section-01/interval-arithmetic/README.md).

## Exercise

Compute `x / x` and `par1(r1, r2)` vs `par2(r1, r2)` on `[9.5, 10.5]` and explain why one bound is wider.

[Parent section](../INTRO.md) · [Chapter 2 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
