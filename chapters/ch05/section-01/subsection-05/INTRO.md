# 5.1.5 Instruction Summary

## What this subsection teaches

Every machine instruction (`assign`, `test`, `branch`, `goto`, `save`, `restore`, `perform`) defines an exact deterministic **state transition** over `(registers, stack, flag, pc)`:
- `assign`/`perform` advance `pc += 1`
- `test` sets boolean `flag` and advances `pc += 1`
- `branch` jumps only when `flag` is true, else advances `pc += 1`
- `goto` unconditionally sets `pc` from a label or register.

## Connecting the concept to Rust and Python

- **In Rust:** Enumerating all 7 variants inside one exhaustive `match` ensures every instruction updates `pc`, `flag`, or `stack` deterministically (`register-simulator`).
- **In Python:** An explicit `if/elif` or `match` dispatch loop over instruction dataclasses checks preconditions (`invalid register`, `stack underflow`) cleanly.
- **Why compare both:** Because instruction semantics are tiny total state transitions, building a reliable simulator only takes ~50 lines in either Rust or Python.

## Exercise

Write the exact `(regs, stack, flag, pc)` transition rule for `branch` when `flag == false` vs `flag == true`.

[Parent section](../INTRO.md) · [Chapter 5 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
