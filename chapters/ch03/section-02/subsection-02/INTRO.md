# 3.2.2 Applying Functions

## What this subsection teaches

When you apply a closure `(params, body, def_env)` to argument values `[v1, ..., vk]`, the evaluator:
1. Allocates a **new frame** binding `params -> values`.
2. Points that new frame's **parent link to `def_env`** (where the function was *created*, NOT where it is *called*!).
3. Evaluates `body` inside that new extended environment.

That single rule is what gives every language **lexical scope**.

## Connecting the concept to Rust and Python

- **In Rust:** Closure structs store `{ param, body, env: usize }` so `apply` always links the newly created call frame back to `closure.env`, ignoring caller locals.
- **In Python:** Python functions store `__closure__` cells pointing back to their creation scope (`def_env`), so calling a returned closure from anywhere still sees its original defining variables.
- **Why compare both:** Both Rust and Python follow static **lexical** scope rather than dynamic call-stack lookup—making `make_withdraw` factories work identically.

## Exercise

Trace calling `f = make_adder(10)` from inside another function that happens to bind its own local `x = 99`.

[Parent section](../INTRO.md) · [Chapter 3 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
