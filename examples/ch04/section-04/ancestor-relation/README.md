# Recursive ancestor relation by graph traversal

**Section:** 4.4 · **Source concept:** shared

## Goal

Turn direct parent facts into a transitive ancestor relation. Repeated rule application discovers facts not explicitly stored.

## Implementation walkthrough

The query uses a stack of candidate descendants and a visited set. A child-to-parent edge is followed until no unseen ancestors remain, protecting against cycles in inconsistent input.

## Complexity and correctness

O(V+E) on the explored portion of the relation graph, using O(V) extra space.

## Run and validate

```sh
cargo run --bin ch04-ancestor-relation
cargo test --bin ch04-ancestor-relation
```

## Extend the example

Return paths as proof witnesses and compare depth-first versus breadth-first traversal.

[Section introduction](../../../../chapters/ch04/section-04/INTRO.md). Source code and explanatory text are original Rust teaching material.
