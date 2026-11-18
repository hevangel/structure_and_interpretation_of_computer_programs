# 1.3.3 Functions as General Methods

## What this subsection teaches

Numerical algorithms like **half-interval bisection** (narrowing an opposite-sign bracket `[a, b]` by half per step) and **fixed-point iteration** (`x_{n+1} = f(x_n)` until `|x_{n+1} - x_n| < tol`) work for *any* continuous function satisfying their preconditions.

Crucially, convergence is part of the method's contract: un-damped `x -> y / x` ping-pongs forever (`g0 -> y/g0 -> g0`), while **average damping** `x -> (x + y/x)/2` contracts toward the true root.

## Connecting the concept to Rust and Python

- **In Rust:** Accepting `f: impl Fn(f64) -> f64` alongside an explicit `Result<f64, &str>` makes precondition failures (same-sign endpoints or max-iteration divergence) explicit at the type level.
- **In Python:** Python functions accept any callable `f: Callable[[float], float]` and raise `ValueError` or `RuntimeError` if endpoints share a sign or iteration fails to converge within the step limit.
- **Why compare both:** Compare [`half-interval-method`](../../../../examples/ch01/section-03/half-interval-method/README.md), [`fixed-point`](../../../../examples/ch01/section-03/fixed-point/README.md), and [`half-interval-filtered`](../../../../examples/ch01/section-03/half-interval-filtered/README.md) across both languages.

## Exercise

Solve `cos(x) = x` by fixed-point iteration, then observe what happens to `x -> 1 + 1/x` (golden ratio) vs `x -> 2/x` without damping.

[Parent section](../INTRO.md) · [Chapter 1 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
