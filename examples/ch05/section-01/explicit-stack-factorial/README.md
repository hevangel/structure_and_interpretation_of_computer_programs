# Factorial with an explicit control stack

**Section:** 5.1 · **Source concept:** shared

## Goal

Expose the pending multiplications behind a recursive factorial process rather than relying on the Rust call stack.

## Implementation walkthrough

Push multipliers while descending to the base case, then pop them in reverse order as saved continuations are resumed. This is an explicit-control implementation.

## Complexity and correctness

O(n) time and O(n) explicit stack memory; compare it with the constant-memory iterative process in Chapter 1.

## Run and validate

```sh
cargo run --bin ch05-explicit-stack-factorial
cargo test --bin ch05-explicit-stack-factorial
```

## Extend the example

Replace the saved multipliers with explicit stack-frame records that include return addresses.

[Section introduction](../../../../chapters/ch05/section-01/INTRO.md). Source code and explanatory text are original Rust teaching material.
