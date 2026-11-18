# 4.1.1 Core of the Evaluator

## What this subsection teaches

At the heart of an interpreter are two mutually recursive functions: **`eval(exp, env)`** (classifies syntax: self-evaluating literals, variables, special forms like `quote`/`if`/`lambda`, or combinations) and **`apply(proc, args)`** (calls primitives directly or evaluates closure bodies inside a newly extended frame).

Notice how little code is actually language semantics once syntax inspection is separated cleanly.

## Connecting the concept to Rust and Python

- **In Rust:** Separates `Expr` syntax nodes from runtime `Value` objects inside `eval(&Expr, &Env) -> Result<Value, String>` and `apply(&Value, Vec<Value>)`.
- **In Python:** Python uses the exact same `eval_expr(exp, env)` and `apply_proc(proc, args)` recursion over guest `@dataclass` expressions—strictly separate from Python's built-in `eval()`.
- **Why compare both:** See [`tiny-evaluator`](../../../../examples/ch04/section-01/tiny-evaluator/README.md) and [`metacircular-evaluator`](../../../../examples/ch04/section-01/metacircular-evaluator/README.md): static `match` vs Python `isinstance` both isolate interpreter errors safely.

## Exercise

Extend `eval` and primitives with subtraction (`-`) and division-by-zero error reporting.

[Parent section](../INTRO.md) · [Chapter 4 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
