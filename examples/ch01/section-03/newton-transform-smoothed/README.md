# First-class returned functions: Newton transform, smoothing, and iterative improvement

**Book concepts:** Section 1.3 (Subsection 1.3.4). **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

Procedures that return newly constructed procedures (`average_damp`, `deriv`, `newton_transform`, `smooth`, `repeated`, and `iterative_improve`) serve as composable algebraic building blocks for numerical solvers.

## Implementation walkthrough

In Rust, returned functions are heap-allocated closure trait objects (`Rc<dyn Fn(f64) -> f64>`) so operators like `average_damp`, `compose`, and `repeated` can capture arbitrary inner functions without exponential monomorphization types. In Python, closures compose directly. `nth_root(x, n)` applies `floor(lg n)` rounds of `average_damp` to guarantee fixed-point convergence for `n >= 2`.

## Complexity and correctness

Composing `n` transformations via `repeated` executes in O(n) evaluation depth per call; Newton's method achieves quadratic convergence near simple roots once inside the basin of attraction.

## Run it

```sh
cargo run --bin ch01-newton-transform-smoothed
cargo test --bin ch01-newton-transform-smoothed
python3 examples/ch01/section-03/newton-transform-smoothed/main.py
python3 -m unittest examples/ch01/section-03/newton-transform-smoothed/main.py
```

The implementation has self-contained assertions and uses only the Rust and Python standard libraries. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Apply `repeated(smooth(f, dx), k)` to a square wave and observe how repeated smoothing attenuates high-frequency corners.

[Section introduction](../../../../chapters/ch01/section-03/INTRO.md) · [Coverage](../../../../COVERAGE.md)
