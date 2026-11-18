# 4.1.3 Evaluator Data Structures

## What this subsection teaches

What runtime objects actually live inside an interpreter? **Predicates** (`is_true`), **procedures/closures** (`params, body, env`), **environment frames** (lookup, `set!`, `define`, frame extension), and—in statement languages—**`ReturnValue` control wrappers** that unwind inner blocks back to the enclosing function call!

## Connecting the concept to Rust and Python

- **In Rust:** Environments use `Rc<RefCell<Frame>>` chains while JavaScript-style returns propagate cleanly via an explicit `EvalControl::Return(Value)` / `Value` enum.
- **In Python:** Python `Frame` objects link `.parent` references while statement unwinding either returns a `ReturnSignal(value)` sentinel or raises a lightweight internal signal caught cleanly at call boundaries.
- **Why compare both:** Compare how `metacircular-evaluator` models Scheme closures vs how [`javascript-parser`](../../../../examples/ch04/section-01/javascript-parser/README.md) models block-scoped `ReturnValue` unwinding in both languages.

## Exercise

Trace what happens when `return 10;` executes inside a nested `if` inside `{ let y = 20; return 10; }`—why does outer state remain unaffected?

[Parent section](../INTRO.md) · [Chapter 4 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
