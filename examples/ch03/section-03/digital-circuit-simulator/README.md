# Event-driven digital circuit simulator

**Book section:** 3.3. **Edition:** shared.

## Concept

Model wires and logic gates using a logical event queue. A signal change can schedule a future change on a dependent gate.

## Rust implementation

Use a priority queue ordered by simulation time and sequence number. When either input of an AND gate changes, schedule its newly computed output after a fixed delay. This is a transport-delay teaching model, not a full HDL simulator.

## Complexity and invariants

O(E log E) time for E scheduled events and O(E) queue storage. The simulator preserves nondecreasing event timestamps and deterministic tie breaking.

## Run and test

```sh
cargo run --bin ch03-digital-circuit-simulator
cargo test --bin ch03-digital-circuit-simulator
```

## Extension exercise

Add an inverter and two cascaded gates. Contrast transport and inertial delay.

This is independently authored Rust teaching code, not a verbatim textbook translation.

[Section introduction](../../../../chapters/ch03/section-03/INTRO.md)
