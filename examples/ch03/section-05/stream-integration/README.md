# Numerical integration as a lazy stream

**Book section:** 3.5. **Edition:** 1996-scheme.

## Concept

A numerical differential equation can generate successive approximations on demand. Only the requested prefix is computed.

## Rust implementation

Implement Iterator for a tiny Euler solver of dy/dt=y, starting at y(0)=1. The iterator exposes (time, value) pairs and updates its internal state after yielding each item.

## Complexity and invariants

Each step is O(1); Euler's global error is O(step size) for a smooth equation over a fixed interval. The iterator stores constant state.

## Run and test

```sh
cargo run --bin ch03-stream-integration
cargo test --bin ch03-stream-integration
```

## Extension exercise

Implement a midpoint or Runge–Kutta step and compare the error at t=1.

This is independently authored Rust teaching code, not a verbatim textbook translation.

[Section introduction](../../../../chapters/ch03/section-05/INTRO.md)
