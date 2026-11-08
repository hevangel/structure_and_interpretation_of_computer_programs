# 2.5.1 Generic Arithmetic

## What this subsection teaches

Generic operations unify common interfaces across distinct numeric representations. Decide which component owns dispatch and arithmetic rules.

## Connecting the concept to Rust

Use traits, enums, and explicit conversions to expose dispatch and promotion decisions. The translation is intended to explain computational ideas, not imitate the exact source language syntax. Think about the input, output, representation invariant, and what changes when evaluation order or mutable state enters the picture.

## Exercise

Build an addition interface for integers and rationals.

**Study method:** trace a small input on paper, run the closest example from this repository, then explain the result and resource usage. Which aspects are mathematical and which depend on the implementation?

This original study note follows the 1996/2022 [SICP comparison table of contents](https://sicp.sourceacademy.org/). The 1984 original may organize the subject differently.

[Parent section](../INTRO.md) · [Edition notes](../../../../docs/editions.md)
