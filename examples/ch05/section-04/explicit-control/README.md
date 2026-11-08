# Explicit-control expression evaluator

**Book concepts:** Section 5.4. **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

Replace the implicit recursive call stack of an arithmetic evaluator with an explicit data structure.

## Implementation walkthrough

A control register records whether to evaluate an expression or return a value. Frames remember pending right operands and previously evaluated left operands.

## Complexity and correctness

Each AST node is visited O(1) times, and stack space is O(expression depth). The host call stack stays bounded.

## Run it

```sh
cargo run --bin ch05-explicit-control
cargo test --bin ch05-explicit-control
```

The implementation has self-contained assertions and uses only the Rust standard library. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Add multiplication and conditional expressions, writing down the necessary continuation frames.

[Section introduction](../../../../chapters/ch05/section-04/INTRO.md) · [Coverage](../../../../COVERAGE.md)
