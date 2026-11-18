# Accelerated streams, infinite pairs, and delayed-feedback ODE solver

**Book concepts:** Section 3.5 (Subsections 3.5.3–3.5.5). **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

Infinite memoized streams power recursive tableau acceleration (`euler_transform` and `make_tableau` converging on `pi` in 8 terms), lexicographically weighted infinite pairs (producing Ramanujan taxicab numbers `1729` and `4104`), cyclic delayed-feedback ODE integration (`dy/dt = y`), and purely functional stream-based Monte Carlo estimation.

## Implementation walkthrough

`euler_transform` computes Aitken's delta-squared acceleration over alternating partial sums; `make_tableau` recursively applies `euler_transform` to produce an infinite stream of streams whose diagonal yields rapid convergence. `weighted_pairs` merges infinite `(i, j)` pairs ordered by `i^3 + j^3`, exposing adjacent equal sums (`1729`, `4104`). `solve_ode` ties a lazy feedback loop between `y` and `dy = y.map(f)` using a deferred integrand reference.

## Complexity and correctness

Each tableau row improves convergence order dramatically (reducing thousands of Leibniz terms to 8 terms for 10-digit precision on `pi`), while memoized stream cells prevent recomputing earlier rows or feedback steps.

## Run it

```sh
cargo run --bin ch03-stream-acceleration-pairs
cargo test --bin ch03-stream-acceleration-pairs
python3 examples/ch03/section-05/stream-acceleration-pairs/main.py
python3 -m unittest examples/ch03/section-05/stream-acceleration-pairs/main.py
```

The implementation has self-contained assertions and uses only the Rust and Python standard libraries. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Implement `solve_2nd(a, b, dt, y0, dy0)` for the harmonic oscillator `d^2 y / dt^2 + y = 0` and verify `y(pi/2) ≈ sin(pi/2) = 1`.

[Section introduction](../../../../chapters/ch03/section-05/INTRO.md) · [Coverage](../../../../COVERAGE.md)
