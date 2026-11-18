# 2.4.1 Representing Complex Numbers

## What this subsection teaches

A complex number has two natural coordinate systems:
- **Rectangular `(x, y)`:** `x1 + x2` and `y1 + y2` make addition trivial (`O(1)` trig-free).
- **Polar `(r, theta)`:** `r1 * r2` and `theta1 + theta2` make multiplication trivial.

By exporting four generic selectors—`real_part`, `imag_part`, `magnitude`, `angle`—arithmetic operations work seamlessly whichever coordinate system stores the number.

## Connecting the concept to Rust and Python

- **In Rust:** `struct Rect` and `struct Polar` or two enum variants expose the four selectors cleanly (`atan2`, `hypot`, `cos`, `sin` on `f64`).
- **In Python:** Python either mirrors those two dataclasses (converting with `math.hypot`, `math.atan2`, `math.cos`, `math.sin`) or contrasts directly with Python's built-in `complex` type.
- **Why compare both:** See [`complex-representations`](../../../../examples/ch02/section-04/complex-representations/README.md): both Rust and Python prove `(1 + i) * (1 - i) = 2` consistently across both coordinate systems.

## Exercise

Multiply `(1 + i)(1 - i)` manually in rectangular coordinates and in polar coordinates, checking that magnitude and angle match.

[Parent section](../INTRO.md) · [Chapter 2 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
