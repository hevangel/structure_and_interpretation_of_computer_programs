# 1.2.5 Greatest Common Divisors

## What this subsection teaches

Euclid's algorithm replaces a pair of integers with a remainder pair while preserving their gcd. This is a short example of invariant-driven design.

## Connecting the concept to Rust

Use unit tests and loops to compare process shapes. Rust does not guarantee optimization of tail-recursive function calls. The translation is intended to explain computational ideas, not imitate the exact source language syntax. Think about the input, output, representation invariant, and what changes when evaluation order or mutable state enters the picture.

## Exercise

List all pairs from gcd(206,40).

**Study method:** trace a small input on paper, run the closest example from this repository, then explain the result and resource usage. Which aspects are mathematical and which depend on the implementation?

This original study note follows the 1996/2022 [SICP comparison table of contents](https://sicp.sourceacademy.org/). The 1984 original may organize the subject differently.

[Parent section](../INTRO.md) · [Edition notes](../../../../docs/editions.md)
