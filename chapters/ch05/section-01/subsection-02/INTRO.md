# 5.1.2 Abstraction in Machine Design

## What this subsection teaches

Just as procedures hide helper details behind interfaces, a machine's controller treats `op(rem)` or `op(*)` as black-box primitive hardware buttons.

If a simpler machine only has subtraction (`-`) and comparison (`<`), swapping out primitive `rem` for an inner subtraction loop (`t <- t - b while t >= b`) keeps the outer GCD controller intact—showing **refinement** of hardware primitives.

## Connecting the concept to Rust and Python

- **In Rust:** Primitives live inside a pluggable operation map (`&str -> Box<dyn Fn(&[Val]) -> Val>`), letting you swap primitive implementations without altering controller instructions.
- **In Python:** Primitives live in a dict mapping operator names (`"rem"`, `"-"`) to Python callables cleanly passed to `Machine`.
- **Why compare both:** Swapping one entry in the primitive operation map changes machine abstraction level cleanly without rewriting controller flow in either language.

## Exercise

Rewrite the `rem` operation in `gcd` using only `-` and `<` instructions and compare total executed steps.

[Parent section](../INTRO.md) · [Chapter 5 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
