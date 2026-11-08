# 3.4.1 Time in Concurrent Systems

## What this subsection teaches

Concurrent actions can interleave in many orders, so a result must not rely on one favorable schedule.

## Connecting the concept to Rust

Use Arc<Mutex<T>> when synchronization is needed, while reasoning separately about schedule and atomicity. The translation is intended to explain computational ideas, not imitate the exact source language syntax. Think about the input, output, representation invariant, and what changes when evaluation order or mutable state enters the picture.

## Exercise

Show a lost-update interleaving for two increments.

**Study method:** trace a small input on paper, run the closest example from this repository, then explain the result and resource usage. Which aspects are mathematical and which depend on the implementation?

This original study note follows the 1996/2022 [SICP comparison table of contents](https://sicp.sourceacademy.org/). The 1984 original may organize the subject differently.

[Parent section](../INTRO.md) · [Edition notes](../../../../docs/editions.md)
