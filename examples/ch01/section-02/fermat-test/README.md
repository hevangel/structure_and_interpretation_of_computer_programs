# Fermat primality and Carmichael counterexample

**SICP section:** 1.2 — **Edition applicability:** shared. This is an independently written Rust analogue, not a copied solution.

## What the code teaches

Distinguish a mathematically necessary condition for primality from a sufficient test. Carmichael numbers can fool a Fermat-style check.

## Implementation and invariants

Use u128 intermediate arithmetic for modular multiplication so any u64 operands are safe; calculate powers by squaring. Compare the Fermat witness test with exact trial division.

## Computational cost

One modular exponentiation uses O(log n) modular multiplications. Trial division is O(sqrt n); a Fermat success does not prove primality.

## Try it

```sh
cargo run --bin ch01-fermat-test
cargo test --bin ch01-fermat-test
```

## Extension challenge

Test several prime bases against 561 and explain why stronger primality testing is needed.

[Section guide](../../../../chapters/ch01/section-02/INTRO.md) · [All examples](../../../../index.html)
