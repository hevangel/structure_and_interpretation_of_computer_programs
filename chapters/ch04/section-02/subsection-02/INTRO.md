# 4.2.2 A Lazy Evaluator

## What this subsection teaches

Naive normal order (**call-by-name**) re-evaluates an argument every single time a parameter is used (`double(slow_fn(x))` runs `slow_fn` twice!).

Upgrade `Thunk` to **memoize (call-by-need)**: store `Delayed { expr, env }` initially, and on the very first **`actual_value`** force, evaluate it once and overwrite the state to **`Evaluated(val)`** (dropping `expr` and `env` to save memory!). Caution: mixing `set!`/side effects with call-by-need makes evaluation timing subtle.

## Connecting the concept to Rust and Python

- **In Rust:** `Value::Thunk(Rc<RefCell<ThunkState>>)` mutates in place from `Unevaluated(Expr, Env)` to `Evaluated(Value)` on first `actual_value` (`memoized-thunk`, `lazy-evaluator`).
- **In Python:** A `@dataclass class Thunk` updates `.forced = True`, `.value = v`, and sets `.expr = .env = None` on first `actual_value` call cleanly.
- **Why compare both:** See [`memoized-thunk`](../../../../examples/ch04/section-02/memoized-thunk/README.md) and [`lazy-evaluator`](../../../../examples/ch04/section-02/lazy-evaluator/README.md): single-evaluation assertions verify `id(x)` with side-effect counter increments **exactly once** even when used twice.

## Exercise

Trace `let x = expensive(5)` used three times under eager call-by-value, unmemoized call-by-name, and memoized call-by-need.

[Parent section](../INTRO.md) · [Chapter 4 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
