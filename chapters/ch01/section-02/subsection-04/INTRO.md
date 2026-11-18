# 1.2.4 Exponentiation

## What this subsection teaches

Computing `b^n` by multiplying `b` one step at a time takes `O(n)` steps. Noticing that `b^(2k) = (b^k)^2` (or squaring the base while halving the exponent) cuts work to `O(log n)` multiplications.

An iterative version maintains the single invariant `a * b^n == constant`: when `n` is odd, multiply `a` by `b` and decrement `n`; when `n` is even, square `b` and halve `n`.

## Connecting the concept to Rust and Python

- **In Rust:** Rust's explicit integer types (`u64`/`u128`) require care—`b * b` can overflow if not using wider intermediates (`u128`) or modular reduction (`% m`) at every squaring step.
- **In Python:** Python integers have arbitrary precision by default, so repeated squaring handles huge bases seamlessly, though `pow(b, n, m)` or explicit `while n > 0:` with `% m` still keeps numbers small and fast.
- **Why compare both:** Both implementations in [`fast-exponentiation`](../../../../examples/ch01/section-02/fast-exponentiation/README.md) and [`fast-multiplication`](../../../../examples/ch01/section-02/fast-multiplication/README.md) share the exact same doubling/halving loop invariant.

## Exercise

Verify the invariant `a * b^n == 3^13` at every step of repeated squaring.

[Parent section](../INTRO.md) · [Chapter 1 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
