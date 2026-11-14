# Pascal triangle as a recursive process

**SICP section:** 1.2 — **Edition applicability:** shared. This is an independently written Rust analogue, not a copied solution.

## What the code teaches

Build each row from the previous row. The recurrence demonstrates reuse of smaller results instead of recomputing a branching tree.

## Implementation and invariants

The outer edges are always one. Every interior entry equals the sum of the two entries immediately above it. Use owned vectors for a sequence of complete rows.

## Computational cost

Generating row n requires O(n²) additions and O(n) live memory if only the current row is retained; u64 limits the maximum practical n.

## Try it

```sh
cargo run --bin ch01-pascal-triangle
cargo test --bin ch01-pascal-triangle
```

## Extension challenge

Compute a binomial coefficient with both Pascal iteration and a factorial formula, and compare overflow behavior.

[Section guide](../../../../chapters/ch01/section-02/INTRO.md) · [All examples](../../../../index.html)
