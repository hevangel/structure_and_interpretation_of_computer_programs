# 1.2.5 Greatest Common Divisors

## What this subsection teaches

Euclid's insight is that any divisor common to `a` and `b` is also a divisor of `a % b`, so `gcd(a, b) = gcd(b, a % b)` until `b == 0` yields `a`.

By Lamé's theorem, the worst-case number of remainder steps corresponds to consecutive Fibonacci numbers (`gcd(F_{k+1}, F_k)`), proving `O(log min(a, b))` worst-case complexity.

## Connecting the concept to Rust and Python

- **In Rust:** An iterative `while b != 0 { (a, b) = (b, a % b); }` loop executes in `O(1)` stack space with pure unsigned integer arithmetic.
- **In Python:** Python's tuple unpacking makes the step a single atomic swap `a, b = b, a % b` with identical `O(1)` space and `O(log n)` steps (and works cleanly on arbitrarily big integers).
- **Why compare both:** Because tuple assignment evaluates the right-hand side completely before rebinding in both languages, `(a, b) <- (b, a % b)` requires no error-prone temporary ordering.

## Exercise

Trace every `(a, b)` pair produced by `gcd(206, 40)` and check [`euclidean-gcd`](../../../../examples/ch01/section-02/euclidean-gcd/README.md).

[Parent section](../INTRO.md) · [Chapter 1 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
