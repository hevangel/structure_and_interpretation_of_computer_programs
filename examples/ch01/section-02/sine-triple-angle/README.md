# Sine by recursive argument reduction

**SICP section:** 1.2 — **Edition applicability:** shared. This is an independently written Rust analogue, not a copied solution.

## What the code teaches

Estimate sine for a large angle by repeatedly shrinking its argument and reconstructing the result with the triple-angle identity.

## Implementation and invariants

The approximation sin(x)≈x is used only near zero. Applying p(y)=3y-4y³ repeatedly recovers larger angles; count how many reductions occur.

## Computational cost

O(log(|x|/threshold)) recursive depth for large inputs and constant work per level. Approximation error propagates through the polynomial.

## Try it

```sh
cargo run --bin ch01-sine-triple-angle
cargo test --bin ch01-sine-triple-angle
```

## Extension challenge

Change the base-case threshold and chart the number of recursive calls versus error.

[Section guide](../../../../chapters/ch01/section-02/INTRO.md) · [All examples](../../../../index.html)
