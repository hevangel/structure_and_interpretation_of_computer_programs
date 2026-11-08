# 4.2.1 Normal and Applicative Order

## What this subsection teaches

Eager application evaluates arguments first, while normal order delays them. Divergence and side effects can reveal the difference.

## Connecting the concept to Rust

Use boxed closures and Option caches to make delayed computation and memoization observable. The translation is intended to explain computational ideas, not imitate the exact source language syntax. Think about the input, output, representation invariant, and what changes when evaluation order or mutable state enters the picture.

## Exercise

Give a function call whose unused argument diverges.

**Study method:** trace a small input on paper, run the closest example from this repository, then explain the result and resource usage. Which aspects are mathematical and which depend on the implementation?

This original study note follows the 1996/2022 [SICP comparison table of contents](https://sicp.sourceacademy.org/). The 1984 original may organize the subject differently.

[Parent section](../INTRO.md) · [Edition notes](../../../../docs/editions.md)
