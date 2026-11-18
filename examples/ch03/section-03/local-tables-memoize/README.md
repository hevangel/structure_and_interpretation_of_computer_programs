# 1D/2D/N-D local tables and dynamic memoization

**Book concepts:** Section 3.3 (Subsection 3.3.3). **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

Hierarchical association tables (`Table<K, V>`) generalize 1D and 2D dispatch tables to arbitrary key paths with pluggable equality predicates (Exercise 3.24), and local mutable tables power `memoize` to collapse exponential recursion into linear work (Exercise 3.27).

## Implementation walkthrough

Each `Table` node stores an optional terminal value and a spine of `(key, child_table)` pairs searched via a caller-supplied `same_key` predicate (for example, floating-point epsilon matching). `memoize` wraps a recursive step function with a private dictionary/map so each distinct argument is evaluated once and subsequent lookups return cached results immediately.

## Complexity and correctness

Multi-key lookup and insertion inspect at most `O(d * b)` comparisons for depth `d` and branching factor `b` (or `O(1)` amortized per key under hash equality). Memoized Fibonacci evaluates `0..=n` once each (`n + 1` evaluations total), reducing time from `O(phi^n)` to `O(n)`.

## Run it

```sh
cargo run --bin ch03-local-tables-memoize
cargo test --bin ch03-local-tables-memoize
python3 examples/ch03/section-03/local-tables-memoize/main.py
python3 -m unittest examples/ch03/section-03/local-tables-memoize/main.py
```

The implementation has self-contained assertions and uses only the Rust and Python standard libraries. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Use a 2D `Table` to memoize longest common subsequence (`LCS(i, j)`) and count subproblem evaluations vs. naive recursion.

[Section introduction](../../../../chapters/ch03/section-03/INTRO.md) · [Coverage](../../../../COVERAGE.md)
