# 1.1.8 Black-Box Abstractions

## What this subsection teaches

A black-box function promises behavior without exposing its method. Clients should rely on contracts rather than internal variables or algorithms.

## Connecting the concept to Rust

Use Rust let bindings, fn declarations, if expressions, and pure arithmetic functions to contrast naming with evaluation. The translation is intended to explain computational ideas, not imitate the exact source language syntax. Think about the input, output, representation invariant, and what changes when evaluation order or mutable state enters the picture.

## Exercise

Rewrite a numeric helper without changing callers.

**Study method:** trace a small input on paper, run the closest example from this repository, then explain the result and resource usage. Which aspects are mathematical and which depend on the implementation?

This original study note follows the 1996/2022 [SICP comparison table of contents](https://sicp.sourceacademy.org/). The 1984 original may organize the subject differently.

[Parent section](../INTRO.md) · [Edition notes](../../../../docs/editions.md)
