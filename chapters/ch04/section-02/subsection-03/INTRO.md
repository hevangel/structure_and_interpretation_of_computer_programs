# 4.2.3 Streams as Lazy Lists

## What this subsection teaches

A lazy list has an available head and a delayed tail, enabling finite access to unbounded structures.

## Connecting the concept to Rust

Use boxed closures and Option caches to make delayed computation and memoization observable. The translation is intended to explain computational ideas, not imitate the exact source language syntax. Think about the input, output, representation invariant, and what changes when evaluation order or mutable state enters the picture.

## Exercise

Request only ten elements of an infinite sequence.

**Study method:** trace a small input on paper, run the closest example from this repository, then explain the result and resource usage. Which aspects are mathematical and which depend on the implementation?

This original study note follows the 1996/2022 [SICP comparison table of contents](https://sicp.sourceacademy.org/). The 1984 original may organize the subject differently.

[Parent section](../INTRO.md) · [Edition notes](../../../../docs/editions.md)
