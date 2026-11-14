# Memoization with a mutable table

**SICP:** Section 3.3 | **Concept shared:** shared

## Learning goal

Retain answers to overlapping Fibonacci subproblems instead of recomputing identical branches.

## Algorithm and Rust design

A HashMap caches completed values. Before computing fib(n), check whether a result is already available. Each new n inserts one value after its dependencies are computed.

## Correctness and tradeoffs

O(n) time and O(n) additional memory versus exponential work for naive recursion. u64 values are valid through fib(93), so tests stay below that limit.

## Run it

```sh
cargo run --bin ch03-memoized-fibonacci
cargo test --bin ch03-memoized-fibonacci
```

## Extension problem

Write a bottom-up table algorithm and explain which values are actually necessary to compute fib(n).

[Section notes](../../../../chapters/ch03/section-03/INTRO.md) · This Rust program and explanation are independently written.
