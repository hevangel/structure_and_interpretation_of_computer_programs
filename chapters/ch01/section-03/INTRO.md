# 1.3 Formulating Abstractions with Higher-Order Procedures · [Chapter 1](../README.md)

## The big idea

Whenever three algorithms differ only in *which* term function or combiner they plug into the same summation loop, bisecting search, or fixed-point loop, the shared pattern itself deserves a name. Higher-order functions treat **computations as values**: they accept functions as parameters and return freshly configured functions (like derivatives, Newton transforms, or `n`-fold average damping) as results.

## What you should focus on

Focus on ****higher-order accumulators, closures (`lambda`), bisection & fixed-point convergence, numerical integration (Simpson's rule), continued fractions, and functions returning functions****. The core discipline is separating *what* a computational method guarantees (its contract and invariant) from *how* any one helper is coded.

## Connecting the concept to Rust and Python

- **In Rust:** Functions and closures are parameterized either statically with zero overhead (`fn<F: Fn(f64) -> f64>(f: F)`) or stored/returned dynamically via boxed or reference-counted trait objects (`Rc<dyn Fn(f64) -> f64>` when compositions like `repeated` reuse a closure).
- **In Python:** Every `def` and `lambda` is already a uniform first-class object, so `compose(f, g)` is simply `lambda x: f(g(x))`. Capturing loop variables or mutating counters requires care with late binding (`default args`) or `nonlocal`.
- **Key contrast:** Python requires zero type plumbing to return composed functions (`newton_transform(deriv(f))`), while Rust makes closure ownership (`move`) and sharing (`Rc`) visible in the type signature—letting you choose between zero-cost static inlining and heap-allocated combinators.

## Subsections

- [1.3.1 Functions as Arguments](subsection-01/INTRO.md)
- [1.3.2 Lambda Expressions](subsection-02/INTRO.md)
- [1.3.3 Functions as General Methods](subsection-03/INTRO.md)
- [1.3.4 Functions as Returned Values](subsection-04/INTRO.md)

## Runnable examples in this section

- [1.3 Higher-order accumulation](../../../examples/ch01/section-03/accumulate/README.md)
- [1.3 Function composition and repeated application](../../../examples/ch01/section-03/function-composition/README.md)
- [1.3 Bisection and the intermediate value theorem](../../../examples/ch01/section-03/half-interval-method/README.md)
- [1.3 Fixed-point iteration](../../../examples/ch01/section-03/fixed-point/README.md)
- [1.3 Higher-order numerical integration](../../../examples/ch01/section-03/simpson-integration/README.md)
- [1.3 Continued-fraction approximations](../../../examples/ch01/section-03/continued-fraction/README.md)
- [1.3 Filtered accumulation, half-interval roots, and continued fractions](../../../examples/ch01/section-03/half-interval-filtered/README.md)
- [1.3 First-class returned functions: Newton transform, smoothing, and iterative improvement](../../../examples/ch01/section-03/newton-transform-smoothed/README.md)

## Check your understanding

1. Why does naive fixed-point iteration `x -> y / x` oscillate around `sqrt(y)` while `average_damp` (`x -> (x + y/x)/2`) converges rapidly?
2. Why do higher roots like `x^(1/8)` require multiple applications of `average_damp` (`floor(log2(n))`)?

**Experiment:** Compare the step count of undamped vs damped fixed-point search on `x -> log(1000) / log(x)` and test `repeated(smooth(f), k)`.

[Chapter 1 overview](../README.md) · [Edition notes](../../../docs/editions.md)
