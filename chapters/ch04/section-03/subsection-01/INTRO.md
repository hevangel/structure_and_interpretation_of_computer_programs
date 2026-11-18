# 4.3.1 Amb and Search

## What this subsection teaches

`(amb e1 e2 ... ek)` conceptually returns *one* of `e1..ek`—whichever succeeds without tripping any later `(require cond)` failure! Calling `(amb)` with zero choices **fails immediately**, triggering chronological backtracking to the most recent unresolved choice point.

## Connecting the concept to Rust and Python

- **In Rust:** Backtracking search passes explicit choice slices cleanly or delegates to continuation closures (`amb-evaluator`) without blowing host stack frames indiscriminately.
- **In Python:** Python can model simple `amb` searches via generator loops or explicit `(succeed, fail)` continuations (`amb-evaluator`) that expose exact backtrack order.
- **Why compare both:** Contrast imperative loops (`n-queens-search`) against [`amb-evaluator`](../../../../examples/ch04/section-03/amb-evaluator/README.md), where search logic vanishes from user code.

## Exercise

Trace `(let ((a (amb 1 2 3)) (b (amb 4 5))) (require (= (+ a b) 7)) (list a b))` chronologically.

[Parent section](../INTRO.md) · [Chapter 4 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
