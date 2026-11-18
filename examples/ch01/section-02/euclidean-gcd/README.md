# Euclid's greatest-common-divisor algorithm

**Book concepts:** Section 1.2. **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

Reduce a greatest-common-divisor problem to smaller ones using remainders.

## Implementation walkthrough

Repeatedly replace the pair (a,b) with (b,a mod b). The gcd is invariant under this transformation.

## Complexity and correctness

O(log min(a,b)) Euclidean steps in the worst case for positive inputs; constant additional memory.

## Run it

```sh
cargo run --bin ch01-euclidean-gcd
cargo test --bin ch01-euclidean-gcd
python3 examples/ch01/section-02/euclidean-gcd/main.py
python3 -m unittest examples/ch01/section-02/euclidean-gcd/main.py
```

The implementation has self-contained assertions and uses only the Rust and Python standard libraries. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Prove the remainder identity, then compare the iteration count for successive Fibonacci numbers.

[Section introduction](../../../../chapters/ch01/section-02/INTRO.md) · [Coverage](../../../../COVERAGE.md)
