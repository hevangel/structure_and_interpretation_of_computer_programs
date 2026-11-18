# Memoized lazy streams, sieve, and formal power series

**Book concepts:** Section 3.5 (Subsections 3.5.1 and 3.5.2). **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

True memoized `delay`/`force` streams cache forced tails in place so self-referential infinite streams (`hamming_stream`, `exp_series`, mutually recursive `sin`/`cos` power series, and Eratosthenes' `sieve`) run in polynomial rather than exponential time.

## Implementation walkthrough

Each `Stream` cell stores an eager `head` and a memoized tail slot (`Thunk -> Forced` in Rust via `Rc<RefCell<StreamTail<T>>>`, cached `_forced` reference in Python). Power-series operators (`integrate_series`, `mul_series`, `invert_unit_series`, `div_series`) manipulate infinite Taylor coefficient streams directly, defining `sin`, `cos`, and `tan = sin / cos` co-recursively.

## Complexity and correctness

Because every stream node forces its tail thunk at most once, re-traversing any prefix costs zero extra thunk calls. Computing the first `N` coefficients of `mul_series` or `invert_unit_series` evaluates O(N^2) additions/multiplications instead of blowing up exponentially on shared self-references.

## Run it

```sh
cargo run --bin ch03-memoized-streams-series
cargo test --bin ch03-memoized-streams-series
python3 examples/ch03/section-05/memoized-streams-series/main.py
python3 -m unittest examples/ch03/section-05/memoized-streams-series/main.py
```

The implementation has self-contained assertions and uses only the Rust and Python standard libraries. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Implement the explicit `stream_limit` convergence monitor (`Exercise 3.64`) and apply it to an infinite Taylor series evaluated at `x = 0.5`.

[Section introduction](../../../../chapters/ch03/section-05/INTRO.md) · [Coverage](../../../../COVERAGE.md)
