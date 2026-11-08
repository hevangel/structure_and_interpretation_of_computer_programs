# 3.5.3 The Stream Paradigm

## What this subsection teaches

Map, filter, and merge form declarative stream pipelines. Laziness can reduce storage but change when effects occur.

## Connecting the concept to Rust

Use Iterator for pull-based laziness; add memoization explicitly when it is part of the semantics. The translation is intended to explain computational ideas, not imitate the exact source language syntax. Think about the input, output, representation invariant, and what changes when evaluation order or mutable state enters the picture.

## Exercise

Compare lazy prime filtering with eager collection.

**Study method:** trace a small input on paper, run the closest example from this repository, then explain the result and resource usage. Which aspects are mathematical and which depend on the implementation?

This original study note follows the 1996/2022 [SICP comparison table of contents](https://sicp.sourceacademy.org/). The 1984 original may organize the subject differently.

[Parent section](../INTRO.md) · [Edition notes](../../../../docs/editions.md)
