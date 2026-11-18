# Numerical integration as a lazy stream

**Book concepts:** Section 3.5 (Subsection 3.5.4). **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

A numerical differential equation can generate successive approximations on demand. Only the requested prefix is computed.

## Implementation walkthrough

Implement an Euler solver stream of dy/dt = y, starting at y(0) = 1. The stream yields `(time, value)` pairs lazily and updates its internal state after yielding each item.

## Complexity and correctness

Each step is O(1); Euler's global error is O(step size) for a smooth equation over a fixed interval. The generator stores constant state.

## Run it

```sh
cargo run --bin ch03-stream-integration
cargo test --bin ch03-stream-integration
python3 examples/ch03/section-05/stream-integration/main.py
python3 -m unittest examples/ch03/section-05/stream-integration/main.py
```

The implementation has self-contained assertions and uses only the Rust and Python standard libraries. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Implement a midpoint or Runge–Kutta step and compare the error at t = 1.

[Section introduction](../../../../chapters/ch03/section-05/INTRO.md) · [Coverage](../../../../COVERAGE.md)
