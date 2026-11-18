# 4.2.1 Normal and Applicative Order

## What this subsection teaches

In **applicative order**, `(try 0 (/ 1 0))` crashes immediately computing arguments *before* entering `try`—even though the body `if (= a 0) 1 b` never inspects `b`.

In **normal order**, arguments travel unevaluated into compound procedures and are forced **only** when demanded by primitives (`+`, `/`), conditionals (`if`), or the top-level driver.

## Connecting the concept to Rust and Python

- **In Rust:** A dedicated evaluator rule wraps compound-procedure arguments instead of calling `eval` eagerly (`conditional-laziness`, `lazy-evaluator`).
- **In Python:** Mirrors normal-order delay cleanly using thunk objects whenever invoking user-defined guest procedures while keeping host Python primitives eager.
- **Why compare both:** Run [`conditional-laziness`](../../../../examples/ch04/section-02/conditional-laziness/README.md) and [`lazy-evaluator`](../../../../examples/ch04/section-02/lazy-evaluator/README.md) to watch `(unless false 1 (/ 1 0))` succeed safely in both Rust and Python.

## Exercise

Construct a two-argument function whose second argument divides by zero or loops forever, and verify which inputs return cleanly under normal order.

[Parent section](../INTRO.md) · [Chapter 4 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
