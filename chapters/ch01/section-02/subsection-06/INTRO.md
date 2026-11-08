# 1.2.6 Testing for Primality

## What this subsection teaches

A composite number has a factor no larger than its square root. Trial division exploits this fact; faster probabilistic techniques need explicit error guarantees.

## Connecting the concept to Rust

Use unit tests and loops to compare process shapes. Rust does not guarantee optimization of tail-recursive function calls. The translation is intended to explain computational ideas, not imitate the exact source language syntax. Think about the input, output, representation invariant, and what changes when evaluation order or mutable state enters the picture.

## Exercise

Test 1, 2, 97, and 221 and explain each result.

**Study method:** trace a small input on paper, run the closest example from this repository, then explain the result and resource usage. Which aspects are mathematical and which depend on the implementation?

This original study note follows the 1996/2022 [SICP comparison table of contents](https://sicp.sourceacademy.org/). The 1984 original may organize the subject differently.

[Parent section](../INTRO.md) · [Edition notes](../../../../docs/editions.md)
