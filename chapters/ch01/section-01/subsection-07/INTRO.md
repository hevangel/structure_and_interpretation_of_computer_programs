# 1.1.7 Square Roots by Newton's Method

## What this subsection teaches

Mathematical definitions say *what* `sqrt(x)` is (`y >= 0` with `y^2 = x`), whereas an algorithm tells *how* to compute it. Newton's update `guess -> (guess + x / guess) / 2` halves relative error quadratically near a positive root.

The subtle engineering point is the **stopping criterion**: comparing `|guess^2 - x| < 1e-9` fails both on tiny numbers like `1e-12` (stops immediately with wrong root) and huge numbers like `1e20` (mantissa precision cannot shrink absolute residual below `1e-9`), so scaling tolerance relatively—and bounding iterations—is essential.

## Connecting the concept to Rust and Python

- **In Rust:** Floating-point (`f64`) has 53 bits of significand precision; combining relative error check `(guess * guess - x).abs() <= eps * x.abs().max(1e-12)` with an explicit step cap guarantees termination.
- **In Python:** Python `float` is the exact same IEEE-754 double (`f64`), so the same relative-tolerance & iteration-bound rule prevents infinite loops at extreme magnitudes.
- **Why compare both:** Since both Rust `f64` and Python `float` share IEEE-754 hardware representation, numerical stability hazards are identical—see [`newton-sqrt`](../../../../examples/ch01/section-01/newton-sqrt/README.md) and [`cube-root-newton`](../../../../examples/ch01/section-01/cube-root-newton/README.md).

## Exercise

Test `sqrt(1e-12)` and `sqrt(1e12)` comparing an absolute residual threshold against a relative change threshold.

[Parent section](../INTRO.md) · [Chapter 1 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
