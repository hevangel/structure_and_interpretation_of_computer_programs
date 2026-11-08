# Generic arithmetic with a numeric tower

**Book concepts:** Section 2.5. **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

Add numbers from different representations while making coercion policies visible.

## Implementation walkthrough

An enum defines integer, rational, and floating-point variants. Promotion chooses the widest required representation; rational inputs are normalized.

## Complexity and correctness

O(log n) for gcd during rational simplification, O(1) otherwise; i64 multiplication is only safe for bounded inputs.

## Run it

```sh
cargo run --bin ch02-generic-arithmetic
cargo test --bin ch02-generic-arithmetic
```

The implementation has self-contained assertions and uses only the Rust standard library. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Add complex numbers as a fourth level and decide whether every coercion should be automatic.

[Section introduction](../../../..//chapters/ch02/section-05/INTRO.md) · [Coverage](../../../../COVERAGE.md)
