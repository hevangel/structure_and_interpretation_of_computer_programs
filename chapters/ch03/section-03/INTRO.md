# 3.3 Modeling with Mutable Data · [Chapter 3](../README.md)

## The big idea

Mutable pairs (`set-car!`, `set-cdr!`) turn tree-like immutable lists into arbitrary **directed graphs**—possibly with shared subgraphs or cycles! That power unlocks `O(1)` queues and `Deque`s, `O(1)` hash/trie dispatch **tables** (`memoize`), **event-driven digital circuit simulators** (wires + time-bucket agenda), and non-directional **constraint propagators** (`forget_value`, `averager`, `squarer`).

It also introduces graph hazards: aliasing corruption, infinite loops during traversal, and conflicting propagations.

## What you should focus on

Focus on ****mutable pairs & cycle detection (Floyd tortoise/hare), `O(1)` queues & deques, N-D local tables & memoization, discrete-event circuit agendas, and informant-tracked constraint retraction****. Always ask which parts of a system depend on *when* operations run versus *what* values they compute.

## Connecting the concept to Rust and Python

- **In Rust:** Mutable graphs either use arena indices into a `Vec<Slot>` (avoiding borrow-checker cycles cleanly while making cycles explicit) or `Rc<RefCell<Node>>` with `Weak<RefCell<Node>>` back-links (`Deque` / `shared-pairs`).
- **In Python:** Objects, lists, and dicts are freely mutable and cyclic (reclaimed by Python's cycle-detecting GC); callback closures on wires and connectors hold direct object references without ownership annotations.
- **Key contrast:** Notice how arena indices (`cycle-detection-deque`) look identical in Rust and Python, whereas reference-counted graphs (`constraint-propagation`, `digital-circuit-simulator`) force Rust to declare `Rc<RefCell<...>>` while Python leaves aliasing implicit.

## Subsections

- [3.3.1 Mutable List Structure](subsection-01/INTRO.md)
- [3.3.2 Representing Queues](subsection-02/INTRO.md)
- [3.3.3 Representing Tables](subsection-03/INTRO.md)
- [3.3.4 Digital Circuit Simulation](subsection-04/INTRO.md)
- [3.3.5 Constraint Propagation](subsection-05/INTRO.md)

## Runnable examples in this section

- [3.3 Mutable FIFO queue abstraction](../../../examples/ch03/section-03/mutable-queue/README.md)
- [3.3 Bidirectional constraint propagation](../../../examples/ch03/section-03/constraint-propagation/README.md)
- [3.3 Event-driven digital circuit simulator](../../../examples/ch03/section-03/digital-circuit-simulator/README.md)
- [3.3 Aliasing with mutable pairs and weak references](../../../examples/ch03/section-03/shared-pairs/README.md)
- [3.3 Memoization with a mutable table](../../../examples/ch03/section-03/memoized-fibonacci/README.md)
- [3.3 Mutable pair cycle detection and O(1) deque](../../../examples/ch03/section-03/cycle-detection-deque/README.md)
- [3.3 1D/2D/N-D local tables and dynamic memoization](../../../examples/ch03/section-03/local-tables-memoize/README.md)

## Check your understanding

1. How can a 3-cell mutable list return `count_pairs == 3`, `4`, `7`, or loop forever?
2. In constraint propagation, why must each `Connector` remember *which constraint (`informant`)* supplied its value before `forget_value` runs?

**Experiment:** Simulate a 4-bit ripple-carry addition in `digital-circuit-simulator` and inspect why changing an OR gate delay shifts total propagation timestamps.

[Chapter 3 overview](../README.md) · [Edition notes](../../../docs/editions.md)
