# 2.5.1 Generic Arithmetic

## What this subsection teaches

Generic arithmetic routes `add(x, y)`, `sub(x, y)`, `mul(x, y)`, `div(x, y)` through one top-level interface regardless of whether arguments are machine integers, rational fractions, reals, or complex numbers—or even complex numbers whose real/imaginary parts are themselves rationals!

Separating the generic API from package internals lets each number package simplify and construct results cleanly.

## Connecting the concept to Rust and Python

- **In Rust:** In Rust, homogeneous operations use `std::ops::{Add, Mul}`, while multi-representation packages use a tagged `Number` enum or generic `DispatchTable` returning normalized results.
- **In Python:** Python supports generic arithmetic both via operator overloading dunders (`__add__`, `__mul__`) returning `NotImplemented` for unknown types, and explicit `(op, (tag1, tag2))` table dispatch.
- **Why compare both:** Compare [`generic-arithmetic`](../../../../examples/ch02/section-05/generic-arithmetic/README.md) side-by-side: both languages route heterogeneous operands cleanly without callers knowing internal tags.

## Exercise

Sketch what happens when adding a complex number whose rectangular components are themselves rationals instead of floats.

[Parent section](../INTRO.md) · [Chapter 2 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
