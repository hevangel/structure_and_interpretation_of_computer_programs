# Recursive and iterative factorial

**Book concepts:** Section 1.2. **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

Compute the same mathematical factorial with two operationally different processes.

## Implementation walkthrough

The recursive version defers multiplication in stack frames. The loop keeps a running product and a simple invariant: result times the remaining factorial equals the original factorial.

## Complexity and correctness

Both take O(n) time. The recursive version takes O(n) stack space in portable Rust; the loop uses O(1). Values beyond 20! exceed u64.

## Run it

```sh
cargo run --bin ch01-factorial-processes
cargo test --bin ch01-factorial-processes
```

The implementation has self-contained assertions and uses only the Rust standard library. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Add checked arithmetic and report overflow instead of assuming the result fits.

[Section introduction](../../../..//chapters/ch01/section-02/INTRO.md) · [Coverage](../../../../COVERAGE.md)
