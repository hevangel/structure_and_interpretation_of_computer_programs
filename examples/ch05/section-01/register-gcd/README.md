# Design a register-machine controller

**Book concepts:** Section 5.1. **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

Describe Euclid's algorithm through changes to two explicit registers instead of recursive function calls.

## Implementation walkthrough

Keep A and B as the machine state. One control step computes the remainder and rotates the registers, preserving their gcd.

## Complexity and correctness

The machine performs logarithmically many steps with O(1) register storage.

## Run it

```sh
cargo run --bin ch05-register-gcd
cargo test --bin ch05-register-gcd
```

The implementation has self-contained assertions and uses only the Rust standard library. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Draw a controller diagram with a test, remainder operation, register move, and halt.

[Section introduction](../../../../chapters/ch05/section-01/INTRO.md) · [Coverage](../../../../COVERAGE.md)
