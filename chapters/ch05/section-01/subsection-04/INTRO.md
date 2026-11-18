# 5.1.4 A Stack for Recursion

## What this subsection teaches

When `factorial` calls `factorial(n - 1)`, a single fixed register `n` cannot simultaneously hold `5`, `4`, `3`, and `2`!

Adding a **LIFO stack** (`save reg` before recursing, `restore reg` upon returning—always restoring in **reverse order** of saving!) lets a single finite machine run arbitrary-depth recursion cleanly (`explicit-stack-factorial`, `register-simulator`).

## Connecting the concept to Rust and Python

- **In Rust:** An explicit `Vec<Val>` stack paired with `push`/`pop` depth tracking proves `factorial(n)` builds exact `n - 1` saved frames before multiplying back up.
- **In Python:** A Python `list` stack with `append`/`pop` tracks identical pending values and return labels cleanly—independent of Python's call stack.
- **Why compare both:** Notice how `explicit-stack-factorial` in both Rust and Python replaces nested call frames directly with one loop pushing onto an explicit array and one loop popping back out.

## Exercise

Trace all `(n, continue)` pairs pushed and popped onto the machine stack for `factorial(4)` and `fib(4)`.

[Parent section](../INTRO.md) · [Chapter 5 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
