# 2.5.2 Combining Different Types

## What this subsection teaches

Coercion allows operations on mixed types, but promotion may lose information. A useful numeric tower must specify conversion policy.

## Connecting the concept to Rust

Use traits, enums, and explicit conversions to expose dispatch and promotion decisions. The translation is intended to explain computational ideas, not imitate the exact source language syntax. Think about the input, output, representation invariant, and what changes when evaluation order or mutable state enters the picture.

## Exercise

Write a conversion table and mark lossy conversions.

**Study method:** trace a small input on paper, run the closest example from this repository, then explain the result and resource usage. Which aspects are mathematical and which depend on the implementation?

This original study note follows the 1996/2022 [SICP comparison table of contents](https://sicp.sourceacademy.org/). The 1984 original may organize the subject differently.

[Parent section](../INTRO.md) · [Edition notes](../../../../docs/editions.md)
