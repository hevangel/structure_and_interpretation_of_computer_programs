# 5.2.4 Monitoring Machine Performance

## What this subsection teaches

Instrumenting the simulator's `Stack` and fetch loop to count **`total_pushes`**, **`max_depth`**, and **executed instruction count**—plus optional per-step trace hooks—turns theoretical Big-O into exact empirical formulas (`total_pushes(fact n) = 2n - 2`, `max_depth = 2n - 2`!).

Comparing those counts lets us verify compiler optimizations (`preserving`, tail calls) quantitatively.

## Connecting the concept to Rust and Python

- **In Rust:** `Stack` tracks `total_pushes`, `current_depth`, and `max_depth` automatically on every `save`/`restore` (`register-simulator`, `explicit-control`, `register-compiler`).
- **In Python:** Python `Stack` tracks the exact same counters (`total_pushes`, `max_depth`, `instruction_count`), matching Rust's counts down to the integer.
- **Why compare both:** Identical push/depth metrics across both implementations prove that instruction-level resource formulas depend purely on the controller architecture, never on host runtime speed.

## Exercise

Run recursive `fib(5)` and `fact(6)` in [`register-simulator`](../../../../examples/ch05/section-02/register-simulator/README.md) and confirm stack depth matches analytical formulas.

[Parent section](../INTRO.md) · [Chapter 5 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
