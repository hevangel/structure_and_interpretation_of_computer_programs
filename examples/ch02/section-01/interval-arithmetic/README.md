# Arithmetic on intervals

**Book concepts:** Section 2.1. **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

Propagate uncertainty by representing all values between a lower and upper bound.

## Implementation walkthrough

The constructor checks ordering. Multiplication considers four endpoint products; division rejects a divisor interval containing zero.

## Complexity and correctness

All four arithmetic combinations are constant time, but repeated interval operations can enlarge overestimates because dependency information is lost.

## Run it

```sh
cargo run --bin ch02-interval-arithmetic
cargo test --bin ch02-interval-arithmetic
```

The implementation has self-contained assertions and uses only the Rust standard library. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Calculate x/x when x is an interval not containing zero. Explain why the naive result is wider than [1,1].

[Section introduction](../../../..//chapters/ch02/section-01/INTRO.md) · [Coverage](../../../../COVERAGE.md)
