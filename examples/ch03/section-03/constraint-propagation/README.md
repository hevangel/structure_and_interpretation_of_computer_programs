# Retractable constraint propagation (`forget!`, `averager`, `squarer`)

**Book concepts:** Section 3.3 (Subsection 3.3.5). **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

Non-directional constraint networks propagate newly known quantities in any direction, track provenance (`informant`) so `forget_value` retracts only values derived from the forgotten source without erasing independent user constraints, and build expression-style combinators (`c+`, `c*`, `c/`, `cv`, `averager`, `squarer`).

## Implementation walkthrough

Each `Connector` records its current value, the `Informant` that supplied it (`User` or a specific `Constraint` ID), and connected constraint listeners. When a constraint infers a missing value, it tags the connector with its own identity; calling `forget_value` clears downstream inferred connectors and lets remaining constraints recompute from surviving user inputs.

## Complexity and correctness

Propagating or retracting a value traverses each reachable constraint edge in the connected component once (`O(C)` for acyclic topologies) and immediately rejects conflicting assignments as contradictions.

## Run it

```sh
cargo run --bin ch03-constraint-propagation
cargo test --bin ch03-constraint-propagation
python3 examples/ch03/section-03/constraint-propagation/main.py
python3 -m unittest examples/ch03/section-03/constraint-propagation/main.py
```

The implementation has self-contained assertions and uses only the Rust and Python standard libraries. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Attach a `Probe` listener to each connector so every `set_value` and `forget_value` prints a chronological propagation trace.

[Section introduction](../../../../chapters/ch03/section-03/INTRO.md) · [Coverage](../../../../COVERAGE.md)
