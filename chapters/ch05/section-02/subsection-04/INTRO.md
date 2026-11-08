# 5.2.4 Monitoring Machine Performance

## What this subsection teaches

Instruction counts and maximum stack depth expose operational cost beyond the mathematical result.

## Connecting the concept to Rust

Use a program counter and a bounded execution loop to interpret machine instructions. The translation is intended to explain computational ideas, not imitate the exact source language syntax. Think about the input, output, representation invariant, and what changes when evaluation order or mutable state enters the picture.

## Exercise

Instrument Euclid's algorithm with a step counter.

**Study method:** trace a small input on paper, run the closest example from this repository, then explain the result and resource usage. Which aspects are mathematical and which depend on the implementation?

This original study note follows the 1996/2022 [SICP comparison table of contents](https://sicp.sourceacademy.org/). The 1984 original may organize the subject differently.

[Parent section](../INTRO.md) · [Edition notes](../../../../docs/editions.md)
