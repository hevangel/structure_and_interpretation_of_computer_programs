# Bisection and the intermediate value theorem

**SICP section:** 1.3 — **Edition applicability:** shared. This is an independently written Rust analogue, not a copied solution.

## What the code teaches

Find a continuous function's zero when its values at two endpoints have opposite signs.

## Implementation and invariants

Maintain an interval that brackets a root. At every step, retain the half whose endpoints still have different signs. A closed error tolerance controls termination.

## Computational cost

O(log((b-a)/tolerance)) evaluations and constant space; the guarantee requires continuity and a valid bracket.

## Try it

```sh
cargo run --bin ch01-half-interval-method
cargo test --bin ch01-half-interval-method
```

## Extension challenge

Compare bisection with Newton iteration when the derivative is near zero.

[Section guide](../../../../chapters/ch01/section-03/INTRO.md) · [All examples](../../../../index.html)
