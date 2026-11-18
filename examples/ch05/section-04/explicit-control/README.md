# Explicit-control 7-register evaluator with tail-call stack discipline

**Book concepts:** Section 5.4 (Subsections 5.4.1–5.4.4). **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

A full explicit-control state machine running across the canonical seven registers (`exp`, `env`, `val`, `continue`, `proc`, `argl`, `unev`) and an explicit stack with push/depth instrumentation. Restoring `continue` before dispatching the last expression of `ev_sequence` achieves genuine tail recursion (`sum-iter(500, 0)` keeps the exact same stack depth as `sum-iter(10, 0)`), while non-tail-recursive `factorial(n)` follows SICP's analytical push formula `32n - 16` and max stack depth `5n + 3`.

## Implementation walkthrough

Each syntactic form (`if`, `begin`, `lambda`, `define`, `set!`, and application) explicitly saves only the registers needed across sub-evaluations onto the machine stack before setting `continue` and jumping back to `eval_dispatch`.

## Complexity and correctness

Tail-recursive calls recycle their continuation frame in `ev_sequence_last_exp`, yielding `O(1)` stack space on iterative loops while ordinary recursion uses `O(n)` stack frames (`5n + 3`).

## Run it

```sh
cargo run --bin ch05-explicit-control
cargo test --bin ch05-explicit-control
python3 examples/ch05/section-04/explicit-control/main.py
python3 -m unittest examples/ch05/section-04/explicit-control/main.py
```

The implementation has self-contained assertions and uses only the Rust and Python standard libraries. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Add `cond` derived dispatch and measure `(factorial-iter n 1)` stack pushes vs `(factorial n)`.

[Section introduction](../../../../chapters/ch05/section-04/INTRO.md) · [Coverage](../../../../COVERAGE.md)
