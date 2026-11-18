# 1.3.1 Functions as Arguments

## What this subsection teaches

Summing integers `sum_integers(a, b)` and summing cubes `sum_cubes(a, b)` repeat the exact same range traversal and accumulator—only the `term(k)` and step `next(k)` vary.

Factoring out the template into a higher-order function (`accumulate(combiner, null_value, term, a, next, b)`) captures summation, products, and filtered series in a single reusable engine.

## Connecting the concept to Rust and Python

- **In Rust:** Generic type bounds (`fn accumulate<F, N, C>(...) where F: Fn(f64) -> f64, ...`) let Rust inline the passed functions statically with zero virtual-call cost.
- **In Python:** Python accepts any callable directly (`def accumulate(combiner, null_value, term, a, next_fn, b):`) with no signature boilerplate, relying on call-site protocol agreement.
- **Why compare both:** Whether statically monomorphized (Rust) or dynamically dispatched (Python), the interface separates *iteration structure* from *term semantics* identically—see [`accumulate`](../../../../examples/ch01/section-03/accumulate/README.md) and [`simpson-integration`](../../../../examples/ch01/section-03/simpson-integration/README.md).

## Exercise

Express both `factorial(n)` and `pi_sum(a, b)` as single calls to `accumulate`.

[Parent section](../INTRO.md) · [Chapter 1 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
