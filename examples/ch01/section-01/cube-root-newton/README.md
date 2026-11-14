# Cube roots by Newton iteration

**SICP section:** 1.1 — **Edition applicability:** shared. This is an independently written Rust analogue, not a copied solution.

## What the code teaches

Find a cube root through a reusable improvement process, including negative arguments that have real cube roots.

## Implementation and invariants

For positive |x|, the update (2g + |x|/g²)/3 preserves a positive guess. Apply the original sign at the end. Each iteration moves toward a fixed point.

## Computational cost

Constant work and memory per iteration; convergence is quadratic near a nonzero root. A maximum iteration count makes the algorithm bounded.

## Try it

```sh
cargo run --bin ch01-cube-root-newton
cargo test --bin ch01-cube-root-newton
```

## Extension challenge

Compare your cube-root approximation to f64::cbrt at extreme magnitudes.

[Section guide](../../../../chapters/ch01/section-01/INTRO.md) · [All examples](../../../../index.html)
