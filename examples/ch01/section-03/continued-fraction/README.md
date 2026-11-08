# Continued-fraction approximations

**Book concepts:** Section 1.3. **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

Evaluate a finite continued fraction by working backward from its last denominator.

## Implementation walkthrough

Pass numerator and denominator functions. Backward evaluation avoids rebuilding a recursive computation tree.

## Complexity and correctness

O(k) steps and O(1) additional space; denominators must remain nonzero for the chosen input.

## Run it

```sh
cargo run --bin ch01-continued-fraction
cargo test --bin ch01-continued-fraction
```

The implementation has self-contained assertions and uses only the Rust standard library. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Investigate a continued fraction that converges to the golden ratio.

[Section introduction](../../../../chapters/ch01/section-03/INTRO.md) · [Coverage](../../../../COVERAGE.md)
