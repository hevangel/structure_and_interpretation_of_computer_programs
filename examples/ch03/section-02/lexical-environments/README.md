# Lexical scope and environment frames

**Book concepts:** Section 3.2. **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

Lookup follows the chain of enclosing frames; an inner binding shadows rather than replaces its outer counterpart.

## Implementation walkthrough

Each frame holds local names and an optional parent. The evaluator's runtime binding structure is distinct from Rust's compile-time lifetimes.

## Complexity and correctness

Lookup is O(depth) expected time with a hash map per frame. Construction here uses owned parent frames for clarity.

## Run it

```sh
cargo run --bin ch03-lexical-environments
cargo test --bin ch03-lexical-environments
```

The implementation has self-contained assertions and uses only the Rust standard library. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Add assignment that modifies the nearest existing binding without creating a new one.

[Section introduction](../../../../chapters/ch03/section-02/INTRO.md) · [Coverage](../../../../COVERAGE.md)
