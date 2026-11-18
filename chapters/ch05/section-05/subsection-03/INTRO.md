# 5.5.3 Applications and Return Statements

## What this subsection teaches

To compile `(f a1 a2)`: evaluate `f -> proc` (`Next`, preserving `env` and `continue` only if needed), evaluate arguments backward (`a2`, `a1`) building `argl`, then branch on primitive vs compiled procedure!

Look at **`compile_proc_call` tail calls (`linkage == Return`)**: restore `continue` *before* jumping to `compiled_entry`—so compiled recursive tail calls transfer directly (`goto (reg val)`) with **zero stack growth**! In the 2022 JS adaptation, explicit `return` linkages (or host trampolines like `javascript-tail-calls`) manage the same constant-stack discipline.

## Connecting the concept to Rust and Python

- **In Rust:** Handles primitive calls (`apply_primitive_procedure`) and compiled procedure linkage (`Return` direct jump vs `Next`/`Label` saving `continue`) cleanly (`register-compiler`, `javascript-tail-calls`).
- **In Python:** Implements identical primitive vs compiled call dispatch (`compile_procedure_call`), plus a `Bounce`/`trampoline` demo in [`javascript-tail-calls`](../../../../examples/ch05/section-05/javascript-tail-calls/README.md) showing `O(1)` stack tail sums (`1..100_000`) cleanly overcoming Python's `RecursionError`!
- **Why compare both:** Compare `javascript-tail-calls` (`trampoline`) with `register-compiler`: trampolines achieve bounded stack on a standard host call stack, while compiled machine code achieves it natively via `Return` linkage.

## Exercise

Why does compiling `(f x)` with `linkage = Return` need **no** `assign continue (label after_call)` before jumping into `f`?

[Parent section](../INTRO.md) · [Chapter 5 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
