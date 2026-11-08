# Recursive tree folds

**Book concepts:** Section 2.2. **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

Treat nested data as a recursive shape and define the computation by cases on that shape.

## Implementation walkthrough

Leaves contribute a number; branches recursively aggregate their children. The recursive enum uses Vec to give each value a finite size.

## Complexity and correctness

O(nodes) time and O(height) call stack space; no mutation or indexing is needed.

## Run it

```sh
cargo run --bin ch02-recursive-trees
cargo test --bin ch02-recursive-trees
```

The implementation has self-contained assertions and uses only the Rust standard library. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Add a tree map that produces a new tree while preserving its exact branching shape.

[Section introduction](../../../../chapters/ch02/section-02/INTRO.md) · [Coverage](../../../../COVERAGE.md)
