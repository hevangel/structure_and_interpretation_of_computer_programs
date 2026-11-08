# N-queens nondeterministic search

**Book concepts:** Section 4.3. **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

Explore alternatives recursively while rejecting partial queen placements that cannot lead to a complete solution.

## Implementation walkthrough

Each row receives one column; a partial assignment is valid if no earlier queen shares a column or diagonal. Search is explicit backtracking rather than an amb operator.

## Complexity and correctness

Worst-case search is exponential, but pruning prevents much useless exploration. Count all solutions to avoid depending on search order.

## Run it

```sh
cargo run --bin ch04-n-queens-search
cargo test --bin ch04-n-queens-search
```

The implementation has self-contained assertions and uses only the Rust standard library. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Add a generator or callback to stream solutions without retaining them in memory.

[Section introduction](../../../../chapters/ch04/section-03/INTRO.md) · [Coverage](../../../../COVERAGE.md)
