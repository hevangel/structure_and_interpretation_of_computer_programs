# Monte Carlo simulation, Cesàro pi, and resettable PRNG

**Book concepts:** Section 3.1 (Subsection 3.1.2). **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

Encapsulating hidden state inside a random generator (`RandGen` with `Generate` and `Reset`) separates stochastic experiments (`monte_carlo`, Cesàro coprime `pi`, and 2D Monte Carlo integration `estimate_integral`) from state management while keeping tests deterministic.

## Implementation walkthrough

`RandGen` uses a 64-bit linear congruential generator with explicit `Reset(seed)` support (Exercise 3.6). `monte_carlo` runs `trials` independent trials of an arbitrary boolean predicate, supporting both the Cesàro test (`gcd(a, b) == 1` has probability `6 / pi^2`) and `estimate_integral` over a bounding rectangle (`x^2 + y^2 <= 1` estimates `pi`).

## Complexity and correctness

Each trial executes in O(log M) for `gcd` or O(1) for the disk predicate; Monte Carlo standard error decreases as `O(1 / sqrt(trials))`, and resetting the seed reproduces exact sample sequences.

## Run it

```sh
cargo run --bin ch03-monte-carlo-pi
cargo test --bin ch03-monte-carlo-pi
python3 examples/ch03/section-01/monte-carlo-pi/main.py
python3 -m unittest examples/ch03/section-01/monte-carlo-pi/main.py
```

The implementation has self-contained assertions and uses only the Rust and Python standard libraries. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Estimate the area of a tilted ellipse or cardioid with `estimate_integral` and verify convergence across `1_000`, `10_000`, and `100_000` trials.

[Section introduction](../../../../chapters/ch03/section-01/INTRO.md) · [Coverage](../../../../COVERAGE.md)
