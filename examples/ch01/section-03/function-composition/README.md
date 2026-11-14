# Function composition and repeated application

**SICP section:** 1.3 — **Edition applicability:** shared. This is an independently written Rust analogue, not a copied solution.

## What the code teaches

Functions can construct other functions. Composition changes the order of evaluation, and repetition expresses a loop as a higher-order abstraction.

## Implementation and invariants

Use generic Fn bounds for ordinary composition. A returned closure captures the operation and count; the caller can execute it on different inputs.

## Computational cost

Composing two functions adds their costs; applying a function n times takes O(n) invocations.

## Try it

```sh
cargo run --bin ch01-function-composition
cargo test --bin ch01-function-composition
```

## Extension challenge

Implement repeated average-damping for a fixed-point solver and compare convergence.

[Section guide](../../../../chapters/ch01/section-03/INTRO.md) · [All examples](../../../../index.html)
