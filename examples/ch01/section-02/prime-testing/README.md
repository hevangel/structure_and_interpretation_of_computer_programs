# Trial-division primality test

**Book concepts:** Section 1.2. **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

Decide whether a positive integer has a nontrivial divisor, and connect correctness to an upper bound on the search.

## Implementation walkthrough

Check 2 separately, then inspect only odd candidates not larger than the square root, implemented as d <= n/d to avoid overflow.

## Complexity and correctness

O(sqrt(n)) trial divisions in the worst case; constant extra space. This is a teaching algorithm, not a cryptographic test.

## Run it

```sh
cargo run --bin ch01-prime-testing
cargo test --bin ch01-prime-testing
```

The implementation has self-contained assertions and uses only the Rust standard library. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Replace trial division with a probabilistic primality test and document its error model.

[Section introduction](../../../..//chapters/ch01/section-02/INTRO.md) · [Coverage](../../../../COVERAGE.md)
