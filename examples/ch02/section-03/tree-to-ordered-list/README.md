# Tree traversal as ordered-sequence conversion

**Section:** 2.3 · **Edition scope:** shared

## Big idea

An ordered search tree can expose an in-order sequence without clients knowing how nodes are linked.

## Rust design and invariants

Recursively visit left subtree, node, and right subtree. The search tree invariant ensures that values arrive in sorted order.

## Complexity

O(n) time and O(n) output memory; recursion uses O(height) stack.

## Run

```sh
cargo run --bin ch02-tree-to-ordered-list
cargo test --bin ch02-tree-to-ordered-list
```

## Exercise

Rewrite traversal with an explicit stack and analyze worst-case height.

[Undergraduate section guide](../../../../chapters/ch02/section-03/INTRO.md). This code and explanation are original pedagogical work, not a reproduction of the textbook's code.
