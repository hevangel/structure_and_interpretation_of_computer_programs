# 1.1.8 Black-Box Abstractions

## What this subsection teaches

A user of `sqrt(x)` should only depend on its input/output contract, not on `improve`, `good_enough`, or `guess_loop`. Hiding internal helper procedures inside a **lexical block** prevents name collisions and lets internals change freely without breaking callers.

Because internal helpers lie inside the outer scope of `sqrt(x)`, they can even read `x` directly as a **free variable** instead of threading it through every helper call.

## Connecting the concept to Rust and Python

- **In Rust:** Helpers can be nested as `fn` items inside `sqrt` (which cannot directly capture local `let` variables—closures `|g| ...` capture locals) or kept private inside a `mod` while exposing only `pub fn sqrt`.
- **In Python:** Nested `def improve(guess):` inside `def sqrt(x):` naturally captures `x` as a free closure variable and remains completely invisible outside `sqrt`.
- **Why compare both:** Python nested `def`s automatically close over enclosing parameters; Rust distinguishes non-capturing nested `fn`s from environment-capturing closures (`|g| ...`) explicitly.

## Exercise

Refactor [`cube-root-newton`](../../../../examples/ch01/section-01/cube-root-newton/README.md) so helper names remain internal while preserving identical public test behavior.

[Parent section](../INTRO.md) · [Chapter 1 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
