# Higher-order numerical integration

**Book concepts:** Section 1.3. **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

Approximate a definite integral using a function supplied by the caller and Simpson's rule.

## Implementation walkthrough

Divide the domain into an even number of intervals, weighting endpoints once, odd interior points four times, and even interior points twice.

## Complexity and correctness

O(n) evaluations and O(1) extra space. Smoothness assumptions determine truncation error.

## Run it

```sh
cargo run --bin ch01-simpson-integration
cargo test --bin ch01-simpson-integration
```

The implementation has self-contained assertions and uses only the Rust standard library. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Integrate x^4 with successively doubled n and estimate convergence order.

[Section introduction](../../../..//chapters/ch01/section-03/INTRO.md) · [Coverage](../../../../COVERAGE.md)
