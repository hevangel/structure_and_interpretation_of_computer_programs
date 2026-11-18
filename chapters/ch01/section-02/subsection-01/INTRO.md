# 1.2.1 Linear Recursion and Iteration

## What this subsection teaches

Compare two implementations of `n!`:
- **Linear recursive process:** `n * fact(n - 1)` defers `n - 1` pending multiplications until hitting the base case (`4 * (3 * (2 * 1))`), needing `O(n)` stack memory.
- **Linear iterative process:** tracks `(product, counter)` directly (`(1,1) -> (1,2) -> (2,3) -> (6,4) -> (24,5)`), needing only `O(1)` memory—even if stopped mid-flight, all state is sitting in those two variables.

## Connecting the concept to Rust and Python

- **In Rust:** Rust does **not** guarantee tail-call optimization, so writing an iterative process as a recursive tail call still grows the stack on unoptimized builds. Idiomatic Rust expresses iterative processes with `loop`/`while` for guaranteed `O(1)` stack depth.
- **In Python:** Python also lacks tail-call optimization and raises `RecursionError` around 1,000 nested calls, so true `O(1)`-space iterative processes must be written with `while` or `for` loops.
- **Why compare both:** The crucial idea is **process shape**, not source recursion: both languages let you compare deferred stack growth vs explicit state loops cleanly in [`factorial-processes`](../../../../examples/ch01/section-02/factorial-processes/README.md).

## Exercise

Trace all deferred multiplications of `fact_rec(4)` versus the `(product, counter)` pairs of `fact_iter(4)`.

[Parent section](../INTRO.md) · [Chapter 1 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
