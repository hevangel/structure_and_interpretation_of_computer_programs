# 3.5.5 Functional Modularity and Objects

## What this subsection teaches

Immutable event processing and mutable objects express evolving systems differently. Compare how each model handles history and debugging.

## Connecting the concept to Rust

Use Iterator for pull-based laziness; add memoization explicitly when it is part of the semantics. The translation is intended to explain computational ideas, not imitate the exact source language syntax. Think about the input, output, representation invariant, and what changes when evaluation order or mutable state enters the picture.

## Exercise

Design a moving average using a fold and a stateful object.

**Study method:** trace a small input on paper, run the closest example from this repository, then explain the result and resource usage. Which aspects are mathematical and which depend on the implementation?

This original study note follows the 1996/2022 [SICP comparison table of contents](https://sicp.sourceacademy.org/). The 1984 original may organize the subject differently.

[Parent section](../INTRO.md) · [Edition notes](../../../../docs/editions.md)
