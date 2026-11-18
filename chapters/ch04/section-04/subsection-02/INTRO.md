# 4.4.2 How Queries Work

## What this subsection teaches

Simple queries rely on **pattern matching**: walking a query pattern and a database assertion simultaneously while recording consistent `?var -> value` bindings inside a `Frame` (rejecting conflicting bindings).

Rules generalize pattern matching to **two-sided unification**: *both* the query pattern and the rule conclusion may contain variables (`?x`, `?y`)! Unifying `(?a . ?a)` against `(?b . 5)` chains `?a -> ?b` and `?b -> 5` cleanly.

## Connecting the concept to Rust and Python

- **In Rust:** Implements recursive `pattern_match`, `unify_match`, `extend_if_possible`, and `depends_on` cyclic occurs-check on `enum Term` values (`query-unification-engine`).
- **In Python:** Implements identical structural matching and two-sided unification over `Atom | Var | Pair` dataclasses returning updated frame dicts (`None` on conflict or cycle).
- **Why compare both:** Checking the `depends_on` cyclic occurs check (`?x` cannot unify with `(cons 1 ?x)`) prevents infinite substitution loops in both Rust and Python.

## Exercise

Unify `((?x ?y) ?y)` against `((?z 3) ?z)` step by step and check whether the final frame stays consistent.

[Parent section](../INTRO.md) · [Chapter 4 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
