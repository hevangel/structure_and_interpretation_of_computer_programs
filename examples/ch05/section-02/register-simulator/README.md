# Small register-machine simulator

**Book concepts:** Section 5.2. **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

Separate a set of machine instructions from the execution engine that runs them.

## Implementation walkthrough

The machine has three signed registers, a program counter, and explicit branches. A step budget prevents accidental infinite loops.

## Complexity and correctness

Each instruction runs in O(1), so execution time is proportional to the number of executed instructions.

## Run it

```sh
cargo run --bin ch05-register-simulator
cargo test --bin ch05-register-simulator
```

The implementation has self-contained assertions and uses only the Rust standard library. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Add a trace facility and a static verifier that checks branch targets and register indices.

[Section introduction](../../../../chapters/ch05/section-02/INTRO.md) · [Coverage](../../../../COVERAGE.md)
