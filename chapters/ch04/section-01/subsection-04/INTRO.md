# 4.1.4 Running the Evaluator

## What this subsection teaches

Before user programs can run, the evaluator boots a **global environment** populated with **primitive procedures** (`+`, `-`, `*`, `=`, `cons`, `car`, `cdr`, `null?`) and runs a Read–Eval–Print Loop (REPL) or deterministic script driver returning clean results or descriptive error messages.

## Connecting the concept to Rust and Python

- **In Rust:** Populates an initial global `Frame` with named `Value::Primitive(name, fn_ptr)` entries and returns `Result<Value, String>` cleanly on unbound variables or arity mismatches.
- **In Python:** Populates `global_frame` with primitive wrappers `(name, python_callable)` and raises explicit `EvalError` messages without crashing the host test runner.
- **Why compare both:** See [`lexical-closures`](../../../../examples/ch04/section-01/lexical-closures/README.md) and [`metacircular-evaluator`](../../../../examples/ch04/section-01/metacircular-evaluator/README.md): deterministic test suites act like automated scripted REPL sessions.

## Exercise

List three valid test programs and three error conditions (unbound symbol, arity mismatch, calling a non-procedure) and check error handling.

[Parent section](../INTRO.md) · [Chapter 4 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
