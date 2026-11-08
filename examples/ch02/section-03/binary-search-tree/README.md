# Ordered sets as binary search trees

**Book concepts:** Section 2.3. **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

Represent a set by a recursive binary search tree and preserve ordering after insertion.

## Implementation walkthrough

Every left descendant is smaller and every right descendant larger. Repeated inserts ignore duplicates, making set semantics explicit.

## Complexity and correctness

Average search/insert is O(log n) for a balanced tree; this unbalanced version degrades to O(n).

## Run it

```sh
cargo run --bin ch02-binary-search-tree
cargo test --bin ch02-binary-search-tree
```

The implementation has self-contained assertions and uses only the Rust standard library. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Insert ascending integers and measure the resulting tree height, then study rebalancing.

[Section introduction](../../../..//chapters/ch02/section-03/INTRO.md) · [Coverage](../../../../COVERAGE.md)
