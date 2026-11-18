# Event-driven digital circuit simulator and ripple-carry adder

**Book concepts:** Section 3.3 (Subsection 3.3.4). **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

Composable stateful `Wire` objects with subscriber callbacks and a time-ordered `Agenda` queue assemble primitive logic gates (`inverter`, `and_gate`, `or_gate`) into `half_adder`, `full_adder`, and an n-bit `ripple_carry_adder` with deterministic propagation delays.

## Implementation walkthrough

Each `Wire` holds a boolean signal and a list of callback closures triggered whenever its state flips. Gates register callbacks on their input wires and schedule delayed output mutations onto the `Agenda` time-bucket queue (`INVERTER_DELAY = 2`, `AND_DELAY = 3`, `OR_DELAY = 5`). Cascading `half_adder` and `full_adder` components yields an `n`-bit `ripple_carry_adder` verified exhaustively over all 256 pairs of 4-bit inputs.

## Complexity and correctness

Each scheduled transition is queued and dispatched in FIFO order within its timestamp bucket, taking O(log T) per distinct future time slot and guaranteeing causal non-decreasing simulation timestamps.

## Run it

```sh
cargo run --bin ch03-digital-circuit-simulator
cargo test --bin ch03-digital-circuit-simulator
python3 examples/ch03/section-03/digital-circuit-simulator/main.py
python3 -m unittest examples/ch03/section-03/digital-circuit-simulator/main.py
```

The implementation has self-contained assertions and uses only the Rust and Python standard libraries. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Replace the 3-gate `or_gate` delay with De Morgan's `inverter`/`and_gate` construction (`Exercise 3.29`) and verify the new critical-path latency.

[Section introduction](../../../../chapters/ch03/section-03/INTRO.md) · [Coverage](../../../../COVERAGE.md)
