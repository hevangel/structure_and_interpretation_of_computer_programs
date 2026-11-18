# 3.2.4 Internal Definitions

## What this subsection teaches

Inside a procedure body, nested `even(n)` and `odd(n)` often call **each other** mutually cleanly. If internal declarations were evaluated like sequential nested `let`s (`even` bound before `odd` exists), evaluating `even`'s body or calling it during initialization could fail!

Simultaneous block scope pre-allocates *all* local declaration slots in the frame as `*unassigned*` (a Temporal Dead Zone / placeholder) before evaluating their initializer expressions.

## Connecting the concept to Rust and Python

- **In Rust:** Rust allows mutually recursive `fn` items inside any block, whereas local `let` closures cannot capture each other recursively without explicit indirection or pre-allocated evaluator slots.
- **In Python:** Inside Python functions, nested `def` bindings live in the same function frame slot array (`LOAD_DEREF`), so two sibling nested `def` functions (`even` and `odd`) call each other cleanly as long as neither invokes the other *before* both `def` statements execute.
- **Why compare both:** Both [`cse-environment-machine`](../../../../examples/ch03/section-02/cse-environment-machine/README.md) and Chapter 4's `scan_out_defines` make the two-phase *allocate unassigned slots → initialize values* lifecycle explicit.

## Exercise

Explain why mutual recursion fails if inner definitions are desugared naively to sequential `let` bindings without placeholder slots.

[Parent section](../INTRO.md) · [Chapter 3 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
