# 4.3.2 Nondeterministic Examples

## What this subsection teaches

Puzzles like the **five-baker/cooper/fletcher/miller/smith multiple dwelling problem**, **N-Queens**, and **natural-language sentence parsing (`parse-sentence`)** read like pure logical constraints when written with `amb`.

Notice performance still cares about constraint placement: pruning invalid combinations immediately (`require` right after each choice) eliminates massive dead subtrees compared to generating all `5!` tuples before checking!

## Connecting the concept to Rust and Python

- **In Rust:** Demonstrated both directly (`multiple-dwelling`, `n-queens-search`) and inside the interpreted language (`amb-evaluator` solving puzzles & ambiguous DCG sentences).
- **In Python:** Side-by-side Python implementations prune partial constraint violations early and invoke `try-again` continuations to enumerate every valid solution.
- **Why compare both:** Compare [`multiple-dwelling`](../../../../examples/ch04/section-03/multiple-dwelling/README.md), [`n-queens-search`](../../../../examples/ch04/section-03/n-queens-search/README.md), and [`amb-evaluator`](../../../../examples/ch04/section-03/amb-evaluator/README.md) to measure how eager `require` ordering shrinks search steps.

## Exercise

Count how many choice combinations `(require ...)` prunes early in 4-Queens and Multiple Dwelling.

[Parent section](../INTRO.md) · [Chapter 4 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
