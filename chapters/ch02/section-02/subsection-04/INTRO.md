# 2.2.4 The Picture Language

## What this subsection teaches

Picture combinators transform and assemble pictures, producing more pictures. Closure under composition is the key idea, independent of graphics hardware.

## Connecting the concept to Rust

Use Vec, recursive enums, iterators, and pattern matching for structure-preserving transformations. The translation is intended to explain computational ideas, not imitate the exact source language syntax. Think about the input, output, representation invariant, and what changes when evaluation order or mutable state enters the picture.

## Exercise

Compose four small ASCII pictures with beside and above.

**Study method:** trace a small input on paper, run the closest example from this repository, then explain the result and resource usage. Which aspects are mathematical and which depend on the implementation?

This original study note follows the 1996/2022 [SICP comparison table of contents](https://sicp.sourceacademy.org/). The 1984 original may organize the subject differently.

[Parent section](../INTRO.md) · [Edition notes](../../../../docs/editions.md)
