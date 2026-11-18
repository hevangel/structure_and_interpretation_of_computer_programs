# 5.2.3 Instruction Execution Functions

## What this subsection teaches

Wait—if Pass 2 compiles each instruction into a direct execution closure (just like Chapter 4's `analyze`!) instead of matching strings/enums at every step, instruction dispatch runs faster **while reporting malformed register/operation names during assembly** rather than mid-execution!

## Connecting the concept to Rust and Python

- **In Rust:** Either pre-validates register/label indices into typed `Instr` structs or wraps closure executors (`register-simulator`), always guarding against stack underflow or invalid `pc`.
- **In Python:** Python resolves register names, operations, and jump PCs cleanly and raises `ValueError` immediately when assembling unknown operators or restoring an empty stack.
- **Why compare both:** Pre-resolving labels and operation tables during assembly eliminates per-step hash-map lookups in both Rust and Python.

## Exercise

Trigger missing-label, duplicate-label, and stack-underflow errors in [`two-pass-assembler`](../../../../examples/ch05/section-02/two-pass-assembler/README.md) and confirm none panic or silently corrupt state.

[Parent section](../INTRO.md) · [Chapter 5 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
