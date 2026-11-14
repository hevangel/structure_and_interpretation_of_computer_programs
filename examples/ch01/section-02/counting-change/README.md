# Counting coin-change combinations

**SICP section:** 1.2 — **Edition applicability:** shared. This is an independently written Rust analogue, not a copied solution.

## What the code teaches

Count unordered combinations of coins using dynamic programming. This is the same recurrence that a tree-recursive counting process would explore.

## Implementation and invariants

ways[amount] counts combinations using coins considered so far. Iterate the amounts in increasing order so each denomination can be used repeatedly, while each combination is counted once.

## Computational cost

For target T and k denominations, O(kT) time and O(T) memory; the naive branching recursion can take much longer.

## Try it

```sh
cargo run --bin ch01-counting-change
cargo test --bin ch01-counting-change
```

## Extension challenge

Implement the tree-recursive version and count its repeated subproblems; then introduce memoization.

[Section guide](../../../../chapters/ch01/section-02/INTRO.md) · [All examples](../../../../index.html)
