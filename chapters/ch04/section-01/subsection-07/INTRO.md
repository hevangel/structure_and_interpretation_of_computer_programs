# 4.1.7 Analysis versus Execution

## What this subsection teaches

In a naive `eval`, every iteration of a loop `fact(1000)` re-walks the syntax tree, re-checks special forms (`if`, `cond`), and re-expands derived forms (`let -> lambda`) a thousand times.

Separating **`analyze(exp)`** (run *once*, returns a compiled execution function `ExecFn`) from **`execute(env)`** (run *many* times) eliminates syntax-dispatch overhead permanently—a direct bridge from interpretation to compilation!

## Connecting the concept to Rust and Python

- **In Rust:** `analyze(&Expr) -> Result<ExecFn, String>` compiles the tree once into nested `Rc<dyn Fn(&Env) -> Result<Value, String>>` closures (`analysis-then-execution`, `analyzing-evaluator`).
- **In Python:** `analyze(exp) -> Callable[[Frame], Value]` captures already-analyzed child executors directly inside Python closures—zero AST pattern checks occur at runtime.
- **Why compare both:** Run [`analysis-then-execution`](../../../../examples/ch04/section-01/analysis-then-execution/README.md) and [`analyzing-evaluator`](../../../../examples/ch04/section-01/analyzing-evaluator/README.md): instrumentation counters in both Rust and Python confirm syntax analysis runs once while execution runs on every iteration.

## Exercise

Verify why `analyze` count stays constant even as `fact(n)` increases from `n = 1` to `n = 20`.

[Parent section](../INTRO.md) · [Chapter 4 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
