# 4.4.3 Logic and Operational Search

## What this subsection teaches

Is logic programming purely mathematical logic? **Not quite!** Operational details leak through in two classic ways:
1. **Infinite loops:** Left-recursive rules like `(rule (married ?x ?y) (married ?y ?x))` or poorly ordered `and` clauses can diverge forever even when valid answers exist elsewhere.
2. **`not` is negation-as-failure:** `(not (supervisor ?p ?s))` *filters an incoming frame stream*—it works only after earlier clauses bind `?p`, not as an existential generator!

## Connecting the concept to Rust and Python

- **In Rust:** Stream combinators (`and` flatmap, `or` interleave, `not` stream-filter, `unique` deduplication) make the exact evaluation order and filtering behavior transparent.
- **In Python:** List/generator frame pipelines show directly why filtering an empty/unbound frame under `not` drops everything instead of searching the universe.
- **Why compare both:** Testing closed-world `not` and `unique` in [`query-unification-engine`](../../../../examples/ch04/section-04/query-unification-engine/README.md) reveals where declarative rules meet operational search.

## Exercise

Explain why `(and (not (job ?x (programmer))) (address ?x ?a))` produces zero answers while swapping the two clauses succeeds.

[Parent section](../INTRO.md) · [Chapter 4 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
