# 1.3.2 Lambda Expressions

## What this subsection teaches

Giving a one-off helper a global name (`fn add_four(x)`) clutters scope when it is only passed once to `map`, `accumulate`, or `fixed_point`. **Lambda expressions** create anonymous functions right at the point of use—and **let** bindings (`(let ((x a)) body)`) are conceptually just immediate applications of a lambda (`((lambda (x) body) a)`).

## Connecting the concept to Rust and Python

- **In Rust:** Closures (`|x| x + offset`) infer parameter/return types from context and capture `offset` by shared reference or `move` value depending on lifetime needs.
- **In Python:** `lambda x: x + offset` defines an inline single-expression function object; beware that inside a Python loop, lambdas capture variables by **name lookup**, so loop-index lambdas need a default-arg bind (`lambda x, k=k: ...`) to snapshot the current value.
- **Why compare both:** Contrast Rust's explicit capture semantics (`move` vs borrow) against Python's late-binding cell lookup whenever building closures inside loops.

## Exercise

Create an inline closure capturing a constant offset and pass it to [`function-composition`](../../../../examples/ch01/section-03/function-composition/README.md).

[Parent section](../INTRO.md) · [Chapter 1 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
