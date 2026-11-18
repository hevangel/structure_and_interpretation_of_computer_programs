# 5.4.4 Running Explicit Control

## What this subsection teaches

Booting `eceval` wraps the controller loop inside a clean read–eval–print / script driver: initialize `env = global_env`, load `exp = parsed_ast`, set `continue = done`, clear stack stats, run until `done`, and inspect **`val`**, **`total_pushes`**, and **`max_depth`**.

Checking analytical stack formulas (`factorial(n)`: `total_pushes = 32n - 16`, `max_depth = 5n + 3`) reveals every hidden cost of general interpretation—motivating Section 5.5's compiler!

## Connecting the concept to Rust and Python

- **In Rust:** Asserts exact match (`total_pushes = 32n - 16`, `max_depth = 5n + 3` for `n = 1..=6`) in deterministic unit tests (`explicit-control`).
- **In Python:** Asserts the identical `32n - 16` and `5n + 3` invariants in `unittest` across `main.py`.
- **Why compare both:** Because `eceval` re-dispatches AST nodes dynamically (`exp` transitions) on every call, `fact(6)` burns `176` stack pushes and depth `33`—setting a crystal-clear benchmark for Section 5.5's compiler to beat!

## Exercise

Verify why `eceval` performs `32` extra pushes per recursive call of `factorial(n)` by listing which saves occur per frame.

[Parent section](../INTRO.md) · [Chapter 5 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
