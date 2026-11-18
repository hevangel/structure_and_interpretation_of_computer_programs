# 4.1.6 Internal Declarations

## What this subsection teaches

Inside a procedure body, suppose two internal definitions reference each other (`even?(n)` calls `odd?(n - 1)` and vice versa), or recursion doesn't even use `define` at all (**Y-combinator**: `((lambda (f) (f f)) (lambda (f) ...))`).

**`scan_out_defines`** transforms a lambda body so all internal `define` names are pre-bound to `*unassigned*` at frame entry before evaluating initializers sequentially—ensuring simultaneous mutual visibility.

## Connecting the concept to Rust and Python

- **In Rust:** Pre-allocates binding slots as `Value::Unassigned` inside `scan_out_defines` (`metacircular-evaluator`), returning a clear runtime error if read before assignment (the JavaScript Temporal Dead Zone!).
- **In Python:** Mirrors `scan_out_defines` with an `UNASSIGNED` sentinel object in `Frame.bindings`, rejecting premature reads just like Python's `UnboundLocalError` / JS TDZ.
- **Why compare both:** Notice how Python's own compiler uses *the exact same idea*: scanning a function body at compile time for assignments to classify local vs free variables before executing a single line!

## Exercise

Trace Y-combinator factorial `Y(fact_gen)(5)` without any `define` using [`metacircular-evaluator`](../../../../examples/ch04/section-01/metacircular-evaluator/README.md).

[Parent section](../INTRO.md) · [Chapter 4 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
