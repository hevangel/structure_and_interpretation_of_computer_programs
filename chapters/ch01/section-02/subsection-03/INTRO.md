# 1.2.3 Orders of Growth

## What this subsection teaches

**Order of growth** (`Theta(f(n))`) measures how required steps and memory scale as input size `n` grows—ignoring machine clock speed and constant factors so we focus on algorithmic structure.

Crucially, **time growth** and **space growth** can differ dramatically: linear recursive `factorial` uses `Theta(n)` time and `Theta(n)` space; iterative `factorial` uses `Theta(n)` time and `Theta(1)` space; naive `fib` uses `Theta(phi^n)` time yet only `Theta(n)` stack space.

## Connecting the concept to Rust and Python

- **In Rust:** Rust zero-cost abstractions keep constant factors tiny, yet cannot rescue an `O(phi^n)` or `O(2^n)` algorithm at `n = 50`; step-counting instrumented tests make asymptotic separation directly measurable.
- **In Python:** Python has larger per-step interpreter overhead than Rust, making algorithmic order of growth even more visible as `n` grows—`O(log n)` Python easily beats `O(n^2)` compiled loops on large inputs.
- **Why compare both:** Comparing actual step counts (and wall times) between Rust and Python proves why asymptotic complexity dominates hardware speed.

## Exercise

Predict how step count changes when doubling `n` for `O(1)`, `O(log n)`, `O(n)`, `O(n^2)`, and `O(2^n)` algorithms.

[Parent section](../INTRO.md) · [Chapter 1 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
