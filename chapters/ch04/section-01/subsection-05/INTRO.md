# 4.1.5 Data as Programs

## What this subsection teaches

Wait—is there a magic boundary between *data* and *programs*? Evaluating a quoted AST (`(eval '(+ 1 2) env)`) treats **code as ordinary data**! Any system that inspects, transforms, or optimizes an AST (like constant folding or a symbolic evaluator) relies on this equivalence.

Notice also the **Turing / Halting** implication: a universal evaluator (`eval`) can interpret arbitrary programs—and because programs can loop forever, no general checker can decide termination for every AST.

## Connecting the concept to Rust and Python

- **In Rust:** Rust represents guest code strictly as `enum Expr` values (`Value::Pair`/`Expr`) so guest programs can construct, inspect, and pass AST data directly without unsafe JIT tricks.
- **In Python:** Python treats AST dataclasses as first-class objects too—keeping user-constructed guest expressions separate from host Python bytecode (`ast` vs `eval`).
- **Why compare both:** Contrast guest `Expr` data trees in [`metacircular-evaluator`](../../../../examples/ch04/section-01/metacircular-evaluator/README.md) against Chapter 2's symbolic differentiation ASTs: differentiation rewrites expressions, while `eval` executes them.

## Exercise

Write a pure walker over `Expr` that counts all procedure calls inside a nested AST before evaluating it.

[Parent section](../INTRO.md) · [Chapter 4 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
