# 1.1.4 Compound Procedures and Functions

## What this subsection teaches

A compound procedure captures a reusable pattern over parameters—such as `sum_of_squares(a, b) = a*a + b*b`—so callers think in terms of *what* is computed rather than repeating low-level arithmetic.

Once defined, user-defined functions combine just like built-in primitives, letting abstractions stack layer upon layer.

## Connecting the concept to Rust and Python

- **In Rust:** `fn sum_of_squares(a: i64, b: i64) -> i64` locks parameter and return types into an explicit contract checked at every call site; composing two functions compiles down cleanly with zero wrapper overhead.
- **In Python:** `def sum_of_squares(a: float, b: float) -> float:` works uniformly across `int`, `float`, or any type supporting `*` and `+` (with optional type hints checked only by tools, not runtime).
- **Why compare both:** Rust trades extra signature annotations for compile-time domain checks; Python trades static checking for immediate polymorphism across numeric types.

## Exercise

Write an `average(a, b)` helper and compose `mean_square(a, b)` in both Rust and Python.

[Parent section](../INTRO.md) · [Chapter 1 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
