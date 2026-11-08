# Sequence processing and matrix multiplication

**Book concepts:** Section 2.2. **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

Express matrix multiplication as combinations of rows, columns, and reductions.

## Implementation walkthrough

Compute each output cell by a dot product. Shape assertions prevent silent mistakes when dimensions do not match.

## Complexity and correctness

For A of size m×k and B of k×n, the straightforward algorithm takes O(mkn) time and O(mn) output space.

## Run it

```sh
cargo run --bin ch02-matrix-operations
cargo test --bin ch02-matrix-operations
```

The implementation has self-contained assertions and uses only the Rust standard library. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Write a transpose function and implement multiplication using rows of the transposed matrix.

[Section introduction](../../../../chapters/ch02/section-02/INTRO.md) · [Coverage](../../../../COVERAGE.md)
