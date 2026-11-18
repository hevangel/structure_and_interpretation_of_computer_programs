# 3.5.5 Functional Modularity and Objects

## What this subsection teaches

Chapter 3 closes with a deep architectural contrast: **objects with local mutable state** (`withdraw` mutating `balance`) vs **stateless stream processors** (transforming an infinite stream of withdrawal requests directly into a stream of balances, or Monte Carlo trials into a stream of `pi` estimates).

Eliminating assignment makes every intermediate state a reproducible value in a stream—while highlighting that merging *asynchronous* external inputs from multiple agents re-introduces time boundaries.

## Connecting the concept to Rust and Python

- **In Rust:** Contrasts `&mut self` stateful structs against pure iterator/stream folds (`scan` / `zero_crossings` / `monte_carlo_stream`) with deterministic replay.
- **In Python:** Contrasts stateful classes (`nonlocal` / attributes) against pure generator/stream pipelines (`zero-crossings`, `stream_monte_carlo`) where history is explicit data.
- **Why compare both:** See [`zero-crossings`](../../../../examples/ch03/section-05/zero-crossings/README.md) and [`stream-acceleration-pairs`](../../../../examples/ch03/section-05/stream-acceleration-pairs/README.md) to compare pure signal folds against stateful detectors.

## Exercise

Implement a moving average over a stream both purely (pair stream / `scan`) and with a stateful accumulator object, then compare replayability.

[Parent section](../INTRO.md) · [Chapter 3 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
