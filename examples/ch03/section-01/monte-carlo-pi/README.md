# Monte Carlo simulation with explicit pseudo-random state

**SICP:** Section 3.1 | **Concept shared:** shared

## Learning goal

Estimate π through repeated experiments: the fraction of uniformly sampled points in the unit square that lie inside the quarter-circle approaches π/4.

## Algorithm and Rust design

Use a deterministic linear congruential generator. Each trial draws two floating-point values from the generator's current state. A fixed seed makes the experiment reproducible.

## Correctness and tradeoffs

With N independent-like samples, Monte Carlo error is typically O(N^-1/2). The small generator is for pedagogy only, not security or high-quality statistical work.

## Run it

```sh
cargo run --bin ch03-monte-carlo-pi
cargo test --bin ch03-monte-carlo-pi
```

## Extension problem

Run repeated seeds and compare sampling variance with the theoretical 1/sqrt(N) relationship.

[Section notes](../../../../chapters/ch03/section-01/INTRO.md) · This Rust program and explanation are independently written.
