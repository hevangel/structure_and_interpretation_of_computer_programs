# 2.4.1 Representing Complex Numbers

## What this subsection teaches

Cartesian and polar coordinates optimize different operations. A shared interface lets clients add or multiply without depending on storage.

## Connecting the concept to Rust

Use enum variants or traits to keep representation choices separate from abstract operations. The translation is intended to explain computational ideas, not imitate the exact source language syntax. Think about the input, output, representation invariant, and what changes when evaluation order or mutable state enters the picture.

## Exercise

Multiply (1+i)(1-i) using both representations.

**Study method:** trace a small input on paper, run the closest example from this repository, then explain the result and resource usage. Which aspects are mathematical and which depend on the implementation?

This original study note follows the 1996/2022 [SICP comparison table of contents](https://sicp.sourceacademy.org/). The 1984 original may organize the subject differently.

[Parent section](../INTRO.md) · [Edition notes](../../../../docs/editions.md)
