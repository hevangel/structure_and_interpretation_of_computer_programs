# 5.4.2 Sequence and Tail Recursion

## What this subsection teaches

Look at evaluating a procedure body `e1; e2; ...; en` (`ev_sequence`):
- If `en` is **not** the last expression (`e1`), save `unev` and `env`, evaluate `e1`, restore `unev` and `env`, and advance.
- When evaluating the **last** expression `en` (`ev_sequence_last_exp`), **jump directly to `eval_dispatch` WITHOUT saving `continue` or `env`** (since `continue` already holds the caller's return address!).

That single optimization makes `eceval` **tail-recursive**: tail loops use **constant `O(1)` stack space** even when executing recursive syntax!

## Connecting the concept to Rust and Python

- **In Rust:** Demonstrated cleanly in [`explicit-control`](../../../../examples/ch05/section-04/explicit-control/README.md) (`tail_sum(10)` and `tail_sum(500)` both have identical `max_depth == 8`) vs non-tail `fact(n)` (`depth = 5n + 3`).
- **In Python:** Reproduces the exact same constant `max_depth` on `tail_sum(500)` in Python—proving tail-call optimization is a property of the **controller's `save` discipline**, not the host language.
- **Why compare both:** This bridges Chapter 1 (`1.2.1` process shape) and Chapter 5: you finally *see* in registers and stack frames why tail position needs zero stack growth.

## Exercise

Compare `max_depth` and `total_pushes` between recursive `factorial(n)` and tail-recursive `sum_iter(n, 0)` in [`explicit-control`](../../../../examples/ch05/section-04/explicit-control/README.md).

[Parent section](../INTRO.md) · [Chapter 5 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
