# 4.4.1 Deductive Information Retrieval

## What this subsection teaches

Logic programming queries a database of **assertions** (`(supervisor (Bitdiddle Ben) (Warbucks Oliver))`) and **rules** (`(rule (outranked-by ?x ?y) ...)`).

Unlike SQL joins written imperatively or single-output functions, a logic query returns a **stream of all variable substitutions (frames)** consistent with the database—including recursive multi-hop inferences like `outranked-by` or `ancestor`!

## Connecting the concept to Rust and Python

- **In Rust:** Stores rules and facts in explicit slices/vectors and streams matching `Frame` substitutions deterministically (`logic-relations`, `ancestor-relation`, `query-unification-engine`).
- **In Python:** Stores facts and rules as immutable `@dataclass` tuples and yields matching `dict[Var, Term]` substitutions cleanly.
- **Why compare both:** Compare [`logic-relations`](../../../../examples/ch04/section-04/logic-relations/README.md), [`ancestor-relation`](../../../../examples/ch04/section-04/ancestor-relation/README.md), and [`query-unification-engine`](../../../../examples/ch04/section-04/query-unification-engine/README.md) across both languages.

## Exercise

Write a query finding every supervisor chain (`middle -> top`) two hops above a given employee.

[Parent section](../INTRO.md) · [Chapter 4 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
