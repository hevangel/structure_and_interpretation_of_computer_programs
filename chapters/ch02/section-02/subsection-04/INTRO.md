# 2.2.4 The Picture Language

## What this subsection teaches

In Henderson's picture language, the basic unit is a **Painter**—a function that draws an image scaled into *any* geometric **Frame** `(origin, edge1, edge2)` via the affine map `p(x, y) = origin + x*edge1 + y*edge2`.

Because `flip_vert`, `rotate90`, `beside`, `below`, and `square_of_four` transform *frames* and return new **Painters** (closure!), you get stratified visual design completely independent of pixels or ASCII rows.

## Connecting the concept to Rust and Python

- **In Rust:** Painters map cleanly to closures `Rc<dyn Fn(&Frame, &mut Canvas)>` paired with `Vect(f64, f64)` math and deterministic SVG/grid assertions.
- **In Python:** Painters are `Callable[[Frame, Canvas], None]` functions transforming vector frames; `corner_split` and `square_limit` read nearly identically to mathematical notation.
- **Why compare both:** Compare the ASCII grid version [`picture-composition`](../../../../examples/ch02/section-02/picture-composition/README.md) with the full affine frame version [`frame-painters`](../../../../examples/ch02/section-02/frame-painters/README.md).

## Exercise

Verify that composing `rotate90` four times restores every segment coordinate `(x, y)` inside `[0, 1]^2`.

[Parent section](../INTRO.md) · [Chapter 2 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
