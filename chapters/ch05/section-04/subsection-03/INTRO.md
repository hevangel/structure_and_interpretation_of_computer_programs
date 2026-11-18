# 5.4.3 Conditionals and Declarations

## What this subsection teaches

In `ev_if`: save `exp`, `env`, and `continue` first, evaluate the predicate in `exp` with `continue = ev_if_decide`, inspect `val`, restore `exp`/`env`/`continue`, then jump cleanly to `eval_dispatch` on the chosen branch **in tail position**!

Notice the same principle applies to `ev_assignment` (`set!`) and `ev_definition` (`define`): evaluate the value expression while preserving variable name & environment, then mutate the frame in place.

## Connecting the concept to Rust and Python

- **In Rust:** Handles `If`, `Lambda`, `Begin`, `Define`, `Set`, and multi-arg `Call` with explicit `save`/`restore` guards matching standard controller contracts (`explicit-control`).
- **In Python:** Mirrors every controller transition identically (`ev_if`, `ev_if_decide`, `ev_assignment`, `ev_definition`), reporting errors cleanly if assigning an unbound variable.
- **Why compare both:** Notice how evaluating `if`'s chosen branch *without* keeping `continue` saved preserves tail recursion across conditional base cases (`if n == 0 { acc } else { tail_sum(n - 1, acc + n) }`) in both Rust and Python.

## Exercise

Trace which registers `ev_if` saves before evaluating `(if (= x 0) 1 (* x (fact (- x 1))))` and why.

[Parent section](../INTRO.md) · [Chapter 5 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
