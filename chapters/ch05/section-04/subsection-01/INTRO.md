# 5.4.1 Explicit-Control Evaluator Core

## What this subsection teaches

`eceval` turns Chapter 4's recursive `eval`/`apply` into a flat controller operating solely through **7 explicit registers**:
`exp` (AST to evaluate), `env` (current environment), `val` (computed return value), `continue` (next controller label), `proc` (evaluated operator), `argl` (accumulated arg values), and `unev` (remaining unevaluated operand ASTs).

Every sub-evaluation sets `continue` to a return label, jumps to `eval_dispatch`, and receives the result in `val`.

## Connecting the concept to Rust and Python

- **In Rust:** Runs inside an explicit `loop` over `Label` states mutating the 7 named registers and explicit `Stack` (`explicit-control`)—zero Rust stack frames per guest call!
- **In Python:** Runs the exact same explicit controller label loop in Python, tracking `val`, `continue`, `proc`, `argl`, `unev` without nested Python function calls.
- **Why compare both:** Writing `eceval` in Python shows dramatically why explicit-control machines ignore Python's ~1,000 recursion-frame cap cleanly!

## Exercise

List the exact role of all 7 registers right before calling a 2-argument primitive `(+ 3 4)`.

[Parent section](../INTRO.md) · [Chapter 5 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
