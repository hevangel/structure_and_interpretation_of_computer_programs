# 1.3.4 Functions as Returned Values

## What this subsection teaches

Just as functions can take procedures as arguments, they can **build and return brand-new functions**. For example:
- `deriv(g)(x)` returns `(g(x + dx) - g(x)) / dx`
- `newton_transform(g)` returns `x -> x - g(x) / Dg(x)`
- `compose(f, g)` and `repeated(f, n)` manufacture new transformations algebraically.

Notice how Newton's method becomes a two-line composition of `fixed_point` + `newton_transform(g)`!

## Connecting the concept to Rust and Python

- **In Rust:** Because returning a closure that captures another closure involves nested/unsized types, Rust either uses `impl Fn(f64) -> f64 + 'static` (for single-shape returns) or `Rc<dyn Fn(f64) -> f64>` when recursive combinators like `repeated` chain arbitrary steps.
- **In Python:** In Python every nested `def` or `lambda` returns a uniform callable referencing its captured arguments in closure cells—`def newton_transform(g): return lambda x: x - g(x) / deriv(g)(x)` needs zero boxing annotations.
- **Why compare both:** This contrast in [`newton-transform-smoothed`](../../../../examples/ch01/section-03/newton-transform-smoothed/README.md) highlights one of the cleanest differences between Rust (`Rc<dyn Fn>`) and Python (automatic closure heap objects).

## Exercise

Verify why finding `x^(1/8)` converges with 3 average damps (`floor(log2(8)) = 3`) in [`newton-transform-smoothed`](../../../../examples/ch01/section-03/newton-transform-smoothed/README.md).

[Parent section](../INTRO.md) · [Chapter 1 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
