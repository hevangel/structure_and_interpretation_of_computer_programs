# 5.3.2 Garbage Collection

## What this subsection teaches

Reachability from roots determines which heap nodes must be preserved. A collector marks reachable objects and reclaims the rest.

## Connecting the concept to Rust

Use a Vec<Option<Node>> heap and root-based graph traversal for a small collector. The translation is intended to explain computational ideas, not imitate the exact source language syntax. Think about the input, output, representation invariant, and what changes when evaluation order or mutable state enters the picture.

## Exercise

Create an unreachable cycle and test collection.

**Study method:** trace a small input on paper, run the closest example from this repository, then explain the result and resource usage. Which aspects are mathematical and which depend on the implementation?

This original study note follows the 1996/2022 [SICP comparison table of contents](https://sicp.sourceacademy.org/). The 1984 original may organize the subject differently.

[Parent section](../INTRO.md) · [Edition notes](../../../../docs/editions.md)
