# Fibonacci recurrence and process shape

**Book concepts:** Section 1.2. **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

A direct recurrence branches into overlapping subproblems, while an iterative process carries forward only the last two values.

## Implementation walkthrough

Use checked small values and compare the definitions. The recursive function is intentionally inefficient to expose its computation tree.

## Complexity and correctness

Naive recursion has exponential time and linear stack depth; iterative Fibonacci has O(n) time and O(1) additional memory.

## Run it

```sh
cargo run --bin ch01-fibonacci-processes
cargo test --bin ch01-fibonacci-processes
```

The implementation has self-contained assertions and uses only the Rust standard library. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Memoize the recursive implementation and sketch how the recurrence tree changes.

[Section introduction](../../../../chapters/ch01/section-02/INTRO.md) · [Coverage](../../../../COVERAGE.md)
