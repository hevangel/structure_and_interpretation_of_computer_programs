# Reachability-based mark/sweep garbage collection

**Book concepts:** Section 5.3. **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

Explain garbage collection by tracing all heap objects reachable from roots and discarding unmarked objects.

## Implementation walkthrough

Indices act as toy pointers; each node stores outgoing edges. Marking recursively visits only previously unseen slots, then sweeping frees unmarked slots and resets mark bits.

## Complexity and correctness

Mark and sweep together take O(V+E) for heap objects and references; recursive marking uses O(max depth) stack.

## Run it

```sh
cargo run --bin ch05-mark-sweep-gc
cargo test --bin ch05-mark-sweep-gc
```

The implementation has self-contained assertions and uses only the Rust standard library. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Replace recursive marking with an explicit worklist and test a cycle of objects unreachable from roots.

[Section introduction](../../../../chapters/ch05/section-03/INTRO.md) · [Coverage](../../../../COVERAGE.md)
