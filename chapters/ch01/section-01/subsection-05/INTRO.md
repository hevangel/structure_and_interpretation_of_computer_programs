# 1.1.5 The Substitution Model

## What this subsection teaches

The **substitution model** explains pure function calls by conceptually substituting each formal parameter with its argument across the body (`square(5) -> 5 * 5 -> 25`).

Two strategies differ sharply when an argument is unused or expensive: **applicative order** evaluates arguments *before* substituting (`test(0, loop())` diverges immediately), whereas **normal order** substitutes unevaluated expressions and reduces only when needed (`if 0 == 0 then 0 else loop()` returns `0`).

## Connecting the concept to Rust and Python

- **In Rust:** Rust uses strict **applicative order**. Modeling normal order requires wrapping delayed arguments as closures (`|| expr`) or explicit `Thunk` values inside an evaluator AST.
- **In Python:** Python also uses strict **applicative order**. Delayed arguments can be modeled with zero-argument `lambda: expr` thunks or custom AST thunks, as shown in [`evaluation-orders`](../../../../examples/ch01/section-01/evaluation-orders/README.md).
- **Why compare both:** Substitution is an idealized equational tool—it works identically in both languages **only as long as there is no mutation**; once mutable state enters, Chapter 3 replaces it with the environment model.

## Exercise

Expand `eval_applicative` vs `eval_normal` on `if_zero(0, loop())` and run [`evaluation-orders`](../../../../examples/ch01/section-01/evaluation-orders/README.md).

[Parent section](../INTRO.md) · [Chapter 1 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
