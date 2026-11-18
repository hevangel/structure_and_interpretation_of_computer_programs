# 4.3.3 Implementing Amb

## What this subsection teaches

How do you implement `(amb ...)` and `try-again` cleanly? Compile each expression via `analyze_amb` into a **continuation-passing executor**:
`exec(env, succeed, fail)`
where **`succeed(val, next_fail)`** receives the chosen value *plus a fresh failure continuation `next_fail`* holding remaining alternatives!

On assignment (`set!`), save the old value first, mutate it, and pass a custom `fail2` that restores the old value before invoking `fail`—making backtracking **automatically undo mutation**!

## Connecting the concept to Rust and Python

- **In Rust:** `ExecAmbFn = Rc<dyn Fn(&Env, SucceedFn, FailFn)>` uses `Rc` closure chains so arbitrary backtrack trees compose cleanly without mutable global stacks (`amb-evaluator`).
- **In Python:** Nested Python closures `def try_next(idx): ... succeed(choice, lambda: try_next(idx + 1))` implement continuation-passing backtracking and `permanent-set!` / `if-fail` transparently.
- **Why compare both:** Both `main.rs` and `main.py` in [`amb-evaluator`](../../../../examples/ch04/section-03/amb-evaluator/README.md) implement reversible `set!`, `permanent-set!`, `if-fail`, and `try-again` identically via two closures.

## Exercise

Trace `set!` undo vs `permanent-set!` during a failing branch inside [`amb-evaluator`](../../../../examples/ch04/section-03/amb-evaluator/README.md).

[Parent section](../INTRO.md) · [Chapter 4 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
