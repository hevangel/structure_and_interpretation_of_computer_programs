# 1.1.2 Naming and the Environment

## What this subsection teaches

Giving a value a name (`pi = 3.14159`, `radius = 10`) turns raw constants into a shared vocabulary. An **environment** is the lookup table mapping each active name to its value at that point in the program.

When an inner block binds the same name again, **shadowing** hides the outer binding inside that block without altering callers outside it.

## Connecting the concept to Rust and Python

- **In Rust:** `let` bindings are immutable by default and evaluate once at binding time; re-declaring `let x = ...` in a narrower `{ ... }` block creates a brand-new binding with its own memory slot while the outer `x` stays intact.
- **In Python:** Variable names are references stored in frame dictionaries (LEGB: Local, Enclosing, Global, Builtin). Assigning `x = ...` inside a `def` creates a separate local binding unless `nonlocal` or `global` overrides lookup.
- **Why compare both:** Both languages separate *naming a value* from *mutating memory*, though Rust distinguishes shadowing (`let x = ...`) from mutation (`x = ...`) explicitly with `mut`.

## Exercise

Shadow an outer binding `x = 10` inside a nested block/function with `x = 99` and verify the outer `x` stays `10` afterward (see [`evaluation-orders`](../../../../examples/ch01/section-01/evaluation-orders/README.md)).

[Parent section](../INTRO.md) · [Chapter 1 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
