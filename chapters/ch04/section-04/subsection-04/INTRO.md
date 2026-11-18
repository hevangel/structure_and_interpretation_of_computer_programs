# 4.4.4 Implementing Queries

## What this subsection teaches

Inside `qeval`, compound queries combine four orthogonal operations over **streams of frames**:
- **`and`:** pipe each frame out of clause 1 as input into clause 2 (sequential join / flatmap)
- **`or`:** run all branches independently on the same input stream and **interleave** results fairly
- **`not`:** keep frames for which the inner query yields *empty* stream
- **`apply_a_rule`:** **alpha-rename** every variable in the rule (`?x -> (? 7 x)`) using a fresh `rule_counter`, unify conclusion with pattern, and recursively `qeval` the rule body!

## Connecting the concept to Rust and Python

- **In Rust:** `qeval` composes `BTreeMap` frames, `RuleCounter` fresh variable IDs, `unify_match`, and deterministic stream combinators (`query-unification-engine`).
- **In Python:** Mirrors `qeval`, `apply_rule`, `rename_variables`, and `unify_match` concisely in Python dataclasses with full unit tests.
- **Why compare both:** Alpha-renaming variables on every rule invocation is the single crucial invariant stopping recursive `append_to_form` or `outranked_by` calls from clobbering parent rule variables in *both* Rust and Python.

## Exercise

Trace `rename_variables` during two recursive calls of `outranked_by` to verify `?middle` in level 1 stays distinct from `?middle` in level 2.

[Parent section](../INTRO.md) · [Chapter 4 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
