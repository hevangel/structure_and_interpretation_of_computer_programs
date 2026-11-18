# 4.2.3 Streams as Lazy Lists

## What this subsection teaches

Once user-defined functions become non-strict automatically, you no longer need special `delay`/`cons-stream` syntax for infinite streams!

Even **pure procedural `cons`** (`(define (cons x y) (lambda (m) (m x y)))`) automatically delays both `x` and `y` in a lazy evaluator—letting guest code define infinite `ones`, `integers`, and `map` streams with ordinary procedures.

## Connecting the concept to Rust and Python

- **In Rust:** Inside `lazy-evaluator`, defining guest `cons`/`car`/`cdr` as plain interpreted lambdas yields lazy infinite lists tested directly through `actual_value`.
- **In Python:** The exact same guest Scheme/AST `cons`/`car`/`cdr` definitions inside `lazy-evaluator/main.py` construct lazy infinite streams (`integers`, `add_lists`) with zero Python `yield` syntax needed inside the guest program!
- **Why compare both:** Evaluating the *same* procedural `cons` infinite stream inside both Rust and Python lazy evaluators proves laziness belongs to the **evaluator's parameter-passing rule**, not host syntax.

## Exercise

Evaluate `(list-ref integers 10)` inside [`lazy-evaluator`](../../../../examples/ch04/section-02/lazy-evaluator/README.md) and confirm earlier elements are cached cleanly.

[Parent section](../INTRO.md) · [Chapter 4 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
