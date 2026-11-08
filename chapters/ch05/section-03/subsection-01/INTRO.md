# 5.3.1 Memory as Vectors

## What this subsection teaches

Heap objects can be represented as entries in vectors, with indices acting as pointers and explicit root references.

## Connecting the concept to Rust

Use a Vec<Option<Node>> heap and root-based graph traversal for a small collector. The translation is intended to explain computational ideas, not imitate the exact source language syntax. Think about the input, output, representation invariant, and what changes when evaluation order or mutable state enters the picture.

## Exercise

Draw a heap with three connected objects.

**Study method:** trace a small input on paper, run the closest example from this repository, then explain the result and resource usage. Which aspects are mathematical and which depend on the implementation?

This original study note follows the 1996/2022 [SICP comparison table of contents](https://sicp.sourceacademy.org/). The 1984 original may organize the subject differently.

[Parent section](../INTRO.md) · [Edition notes](../../../../docs/editions.md)
