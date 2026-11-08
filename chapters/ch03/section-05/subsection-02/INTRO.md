# 3.5.2 Infinite Streams

## What this subsection teaches

A program can specify an unbounded sequence using finite generator state. The consumer requests only a finite prefix.

## Connecting the concept to Rust

Use Iterator for pull-based laziness; add memoization explicitly when it is part of the semantics. The translation is intended to explain computational ideas, not imitate the exact source language syntax. Think about the input, output, representation invariant, and what changes when evaluation order or mutable state enters the picture.

## Exercise

Generate the first ten square numbers.

**Study method:** trace a small input on paper, run the closest example from this repository, then explain the result and resource usage. Which aspects are mathematical and which depend on the implementation?

This original study note follows the 1996/2022 [SICP comparison table of contents](https://sicp.sourceacademy.org/). The 1984 original may organize the subject differently.

[Parent section](../INTRO.md) · [Edition notes](../../../../docs/editions.md)
