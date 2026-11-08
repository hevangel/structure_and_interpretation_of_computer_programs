# Expression evaluator with lexical bindings

**Book concepts:** Section 4.1. **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

Define evaluation as a recursive algorithm over syntax rather than relying on Rust to interpret a different language.

## Implementation walkthrough

An expression is either a literal, variable, addition, or let-binding. Evaluating a let expression extends a copied environment, preserving the outer binding.

## Complexity and correctness

Evaluation traverses the expression tree; cloning a map per let is simple but adds overhead. Real interpreters use shared environment frames.

## Run it

```sh
cargo run --bin ch04-tiny-evaluator
cargo test --bin ch04-tiny-evaluator
```

The implementation has self-contained assertions and uses only the Rust standard library. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Add lambda expressions and lexical closures, then compare lexical and dynamic scoping.

[Section introduction](../../../../chapters/ch04/section-01/INTRO.md) · [Coverage](../../../../COVERAGE.md)
