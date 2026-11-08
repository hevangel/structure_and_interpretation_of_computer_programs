# 3.4.2 Controlling Concurrency

## What this subsection teaches

Locks and serializers protect invariants across transactions. Choosing the correct atomic scope is more important than locking every individual statement.

## Connecting the concept to Rust

Use Arc<Mutex<T>> when synchronization is needed, while reasoning separately about schedule and atomicity. The translation is intended to explain computational ideas, not imitate the exact source language syntax. Think about the input, output, representation invariant, and what changes when evaluation order or mutable state enters the picture.

## Exercise

Prove a two-account transfer conserves the total.

**Study method:** trace a small input on paper, run the closest example from this repository, then explain the result and resource usage. Which aspects are mathematical and which depend on the implementation?

This original study note follows the 1996/2022 [SICP comparison table of contents](https://sicp.sourceacademy.org/). The 1984 original may organize the subject differently.

[Parent section](../INTRO.md) · [Edition notes](../../../../docs/editions.md)
