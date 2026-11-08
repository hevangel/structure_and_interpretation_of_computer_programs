# Complex numbers with two representations

**Book concepts:** Section 2.4. **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

Support cartesian and polar coordinates through one public abstraction.

## Implementation walkthrough

Use an enum to preserve the selected representation. Addition uses Cartesian components while multiplication composes polar magnitudes and angles.

## Complexity and correctness

Each operation takes O(1) time. Floating-point rounding means results should be compared with a tolerance.

## Run it

```sh
cargo run --bin ch02-complex-representations
cargo test --bin ch02-complex-representations
```

The implementation has self-contained assertions and uses only the Rust standard library. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Add subtraction and division without exposing representation choices to clients.

[Section introduction](../../../..//chapters/ch02/section-04/INTRO.md) · [Coverage](../../../../COVERAGE.md)
