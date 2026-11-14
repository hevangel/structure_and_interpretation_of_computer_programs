# Signal processing with adjacent stream samples

**SICP:** Section 3.5 | **Concept shared:** shared

## Learning goal

Identify sign changes in a sensor stream without needing to retain all earlier measurements.

## Algorithm and Rust design

Each new reading is compared with its predecessor. Zero is treated as a nonpositive sample, so a transition 0→positive is an upward crossing while a transition positive→0 is not yet a downward crossing.

## Correctness and tradeoffs

O(n) time and O(k) output storage for k reported crossings; the streaming state itself is constant size.

## Run it

```sh
cargo run --bin ch03-zero-crossings
cargo test --bin ch03-zero-crossings
```

## Extension problem

Add a hysteresis threshold to reduce repeated crossings under noisy signals.

[Section notes](../../../../chapters/ch03/section-05/INTRO.md) · This Rust program and explanation are independently written.
