# Two-pass assembler with symbolic jump labels

**Section:** 5.2 · **Source concept:** shared

## Goal

Turn symbolic labels into numeric instruction addresses before running a register-machine program.

## Implementation walkthrough

A first pass counts executable instructions and records labels. A second pass emits opcodes and resolves jumps using the map. Duplicated or unknown labels are reported as errors.

## Complexity and correctness

Assembly is O(n) expected time for n source entries. Execution is bounded by a step budget to reject loops that never halt.

## Run and validate

```sh
cargo run --bin ch05-two-pass-assembler
cargo test --bin ch05-two-pass-assembler
```

## Extend the example

Add support for unconditional calls with a saved return-address stack.

[Section introduction](../../../../chapters/ch05/section-02/INTRO.md). Source code and explanatory text are original Rust teaching material.
