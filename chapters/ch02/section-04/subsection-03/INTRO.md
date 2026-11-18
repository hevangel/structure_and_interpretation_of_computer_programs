# 2.4.3 Data-Directed Programming

## What this subsection teaches

Centralized `if`/`match` dispatch breaks down as modules grow: adding every new representation forces modifying core dispatch functions. **Data-directed programming** inverts this using a 2D table keyed by `(operation_name, type_tags)`.

Each module registers its own handlers (`put("add", ["rect", "rect"], ...)`), and `apply_generic(op, args)` simply looks up the combined tag signature—or delegates to **message-passing** closures where values carry their own methods.

## Connecting the concept to Rust and Python

- **In Rust:** Implemented with a `BTreeMap<(String, Vec<String>), Rc<dyn Fn(&[Datum]) -> Result<Datum, ...>>>` registry (as in [`data-directed-dispatch`](../../../../examples/ch02/section-04/data-directed-dispatch/README.md)), contrasted cleanly against static Rust traits.
- **In Python:** Implemented naturally with a nested dictionary `table[(op, type_tags)] = fn` alongside duck-typed classes or message-passing closures `def make_rect(x, y): return lambda op: ...`.
- **Why compare both:** Data-directed tables make adding *new types* zero-edit in both languages—and show why **expression-problem** tradeoffs exist between OOP (easy new types) and FP `match` (easy new operations).

## Exercise

Add a `"div"` rule or exponent differentiation package to [`data-directed-dispatch`](../../../../examples/ch02/section-04/data-directed-dispatch/README.md) without modifying `apply_generic`.

[Parent section](../INTRO.md) · [Chapter 2 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
