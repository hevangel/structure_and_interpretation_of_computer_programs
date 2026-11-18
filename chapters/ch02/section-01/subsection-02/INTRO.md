# 2.1.2 Abstraction Barriers

## What this subsection teaches

An **abstraction barrier** splits a system into horizontal layers: client programs talk only to high-level constructors/selectors, which talk to mid-level operations, which talk to concrete storage.

If `Rect` changes from `(top_left, width, height)` to `(p1, p2)` diagonal points—or `Rat` defers `gcd` reduction until `numer()`/`denom()` are read—no code above the barrier needs a single edit.

## Connecting the concept to Rust and Python

- **In Rust:** Visibility (`pub` vs private fields) lets the Rust compiler reject any caller that tries to reach across the abstraction barrier directly.
- **In Python:** Python marks internal attributes with `_leading_underscore` and uses `@property` methods so callers access `rect.width` identically whether stored directly or computed from endpoints.
- **Why compare both:** Rust enforces abstraction barriers **statically** (compiler error); Python enforces them **by API convention and properties** (`@property`), which still preserves drop-in flexibility.

## Exercise

Compare the `Segment` and rectangle abstractions in [`geometric-segments`](../../../../examples/ch02/section-01/geometric-segments/README.md) across Rust and Python.

[Parent section](../INTRO.md) · [Chapter 2 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
