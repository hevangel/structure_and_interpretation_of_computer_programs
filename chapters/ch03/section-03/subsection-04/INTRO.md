# 3.3.4 Digital Circuit Simulation

## What this subsection teaches

An **event-driven digital circuit simulator** has three cooperating pieces:
1. **Wires:** hold a boolean signal and a list of callback actions notified whenever the signal flips.
2. **Logic gates (`inverter`, `and_gate`, `or_gate`):** subscribe callbacks to input wires and schedule delayed output updates onto an **Agenda** (`after_delay(delay, action)`).
3. **Agenda:** priority/time-bucket queue advancing **logical simulation time** step by step—building up into `half_adder`, `full_adder`, and `n`-bit `ripple_carry_adder`.

## Connecting the concept to Rust and Python

- **In Rust:** `Rc<RefCell<WireState>>` holds wire signal and `Vec<Box<dyn FnMut(&mut Agenda)>>` subscribers, while `Agenda` uses a `BTreeMap<u64, VecDeque<Box<dyn FnOnce(&mut Agenda)>>>` for deterministic chronological execution.
- **In Python:** `Wire` objects store `.signal` and `.actions: list[Callable[[], None]]`, and `Agenda` uses a sorted dict or `heapq` time-slot queue—reading cleanly without `Rc`/`RefCell` wrapping.
- **Why compare both:** In [`digital-circuit-simulator`](../../../../examples/ch03/section-03/digital-circuit-simulator/README.md), both simulators propagate every `4`-bit `(a + b) % 16` ripple-carry addition with exact simulated nanosecond timestamps.

## Exercise

Trace a `half_adder(a=1, b=1)` event by event through the `Agenda` timestamps (`INVERTER=2`, `AND=3`, `OR=5`).

[Parent section](../INTRO.md) · [Chapter 3 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
