# 3.3.2 Representing Queues

## What this subsection teaches

A FIFO queue exposes insertion at the rear and removal at the front. A representation invariant makes these operations reliable.

## Connecting the concept to Rust

Use VecDeque, HashMap, and optional Rc<RefCell<T>> sharing to model mutable structures. The translation is intended to explain computational ideas, not imitate the exact source language syntax. Think about the input, output, representation invariant, and what changes when evaluation order or mutable state enters the picture.

## Exercise

Trace enqueue, enqueue, dequeue on an empty queue.

**Study method:** trace a small input on paper, run the closest example from this repository, then explain the result and resource usage. Which aspects are mathematical and which depend on the implementation?

This original study note follows the 1996/2022 [SICP comparison table of contents](https://sicp.sourceacademy.org/). The 1984 original may organize the subject differently.

[Parent section](../INTRO.md) · [Edition notes](../../../../docs/editions.md)
