# Higher-order accumulation

**Book concepts:** Section 1.3. **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

Recognize summation and product as instances of the same repeated combination pattern.

## Implementation walkthrough

Pass the term mapping, combiner, and identity to a generic procedure. Type parameters keep this particular example numeric, while the pattern itself is more general.

## Complexity and correctness

O(n) calls to the supplied functions and O(1) extra space for the loop-based fold.

## Run it

```sh
cargo run --bin ch01-accumulate
cargo test --bin ch01-accumulate
```

The implementation has self-contained assertions and uses only the Rust standard library. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Generalize the abstraction to a slice fold that can return a different type from its inputs.

[Section introduction](../../../../chapters/ch01/section-03/INTRO.md) · [Coverage](../../../../COVERAGE.md)
