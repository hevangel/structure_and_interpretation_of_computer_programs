# Multiplication by doubling and halving

**SICP section:** 1.2 — **Edition applicability:** shared. This is an independently written Rust analogue, not a copied solution.

## What the code teaches

Use repeated doubling and halving rather than performing one addition per multiplier unit.

## Implementation and invariants

Maintain result + a*b as an invariant. When b is odd, transfer one copy of a into the result; then double a and halve b.

## Computational cost

O(log b) additions and O(1) auxiliary memory. Operands are deliberately bounded to avoid overflow in u64 arithmetic.

## Try it

```sh
cargo run --bin ch01-fast-multiplication
cargo test --bin ch01-fast-multiplication
```

## Extension challenge

Add checked arithmetic that returns an error when the result cannot fit in u64.

[Section guide](../../../../chapters/ch01/section-02/INTRO.md) · [All examples](../../../../index.html)
