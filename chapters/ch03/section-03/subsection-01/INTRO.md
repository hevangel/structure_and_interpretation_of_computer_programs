# 3.3.1 Mutable List Structure

## What this subsection teaches

Reassigning `car` or `cdr` pointers transforms linked chains into arbitrary **graphs** (`x -> y`, shared diamonds where two branches point to the same cell, or closed cycles!).

Sharing breaks naive tree assumptions: a 3-cell structure can look like length `3`, `4`, or `7` to `count_pairs`, or send traversal into an infinite loop unless detected using a **visited set** or **Floyd's `O(1)`-space tortoise-and-hare pointer race**.

## Connecting the concept to Rust and Python

- **In Rust:** Because safe Rust forbids multiple mutable borrows simultaneously, arena indices (`Vec<ConsCell>`) or `Rc<RefCell<Cell>>` model shared mutable pairs cleanly (`shared-pairs`, `cycle-detection-deque`).
- **In Python:** Python objects and lists allow aliasing and self-referential cycles directly (`a[1] = a`), making Floyd's slow/fast two-pointer check directly testable.
- **Why compare both:** Compare [`shared-pairs`](../../../../examples/ch03/section-03/shared-pairs/README.md) and [`cycle-detection-deque`](../../../../examples/ch03/section-03/cycle-detection-deque/README.md): Rust's `Weak` references prevent leak cycles in reference counting, while Python relies on a cycle collector.

## Exercise

Build 3-cell graphs that cause naive `count_pairs` to return `3`, `4`, `7`, and loop, then detect the cycle with Floyd's `O(1)`-space algorithm.

[Parent section](../INTRO.md) · [Chapter 3 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
