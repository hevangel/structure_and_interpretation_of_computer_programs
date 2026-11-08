# Relational queries with variable bindings

**Book concepts:** Section 4.4. **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

Ask a relation to find values that satisfy constraints rather than calculating one fixed output.

## Implementation walkthrough

A pattern term is a constant or named variable. The unifier either preserves a consistent variable binding or rejects a candidate fact.

## Complexity and correctness

Scanning f facts is O(f) expected time per query with small hash maps. A full logic engine adds recursive rules, indexing, and backtracking substitutions.

## Run it

```sh
cargo run --bin ch04-logic-relations
cargo test --bin ch04-logic-relations
```

The implementation has self-contained assertions and uses only the Rust standard library. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Join two parent relations to infer grandparent facts and explore variable-name scoping.

[Section introduction](../../../../chapters/ch04/section-04/INTRO.md) · [Coverage](../../../../COVERAGE.md)
