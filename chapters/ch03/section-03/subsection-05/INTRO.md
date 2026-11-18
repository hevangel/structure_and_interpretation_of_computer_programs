# 3.3.5 Constraint Propagation

## What this subsection teaches

Ordinary functions flow one way (`input -> output`). A **constraint network** connects `Connector` nodes via **multi-directional constraints** (`adder(a, b, sum)`, `multiplier(p1, p2, prod)`, `constant(c, x)`, `squarer(a, b)`): setting *any two* computes the third—whether forward (`C -> F`) or backward (`F -> C`)!

Critically, each connector tracks its **informant** (user vs specific constraint ID) so **`forget_value`** only retracts values downstream of the forgotten constraint without erasing independent user constraints.

## Connecting the concept to Rust and Python

- **In Rust:** Implemented with shared `Rc<RefCell<ConnectorState>>` connectors storing `value`, `informant: Option<usize>`, and `Weak` or ID-indexed constraint listeners (`constraint-propagation`).
- **In Python:** `Connector` classes hold `.value`, `.informant`, and `.constraints` lists notifying `process_new_value()` and `process_forget_value()` across arbitrary graph edges.
- **Why compare both:** See [`constraint-propagation`](../../../../examples/ch03/section-03/constraint-propagation/README.md): notice how tracking `informant` provenance prevents contradicting constraints and enables clean retraction in both Rust and Python.

## Exercise

Build `averager(a, b, avg)` from `adder` + `multiplier` + `constant(2)`, set `a = 10, avg = 15`, verify `b = 20`, then retract `a` and set `b = 30`.

[Parent section](../INTRO.md) · [Chapter 3 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
