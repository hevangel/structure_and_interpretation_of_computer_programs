# 5.1.2 Abstraction in Machine Design

## What this subsection teaches

A machine may treat primitive operations abstractly while its controller remains explicit. Different implementations can preserve the same machine interface.

## Connecting the concept to Rust

Use an instruction enum and named registers to design a small control machine. The translation is intended to explain computational ideas, not imitate the exact source language syntax. Think about the input, output, representation invariant, and what changes when evaluation order or mutable state enters the picture.

## Exercise

Replace remainder with repeated subtraction.

**Study method:** trace a small input on paper, run the closest example from this repository, then explain the result and resource usage. Which aspects are mathematical and which depend on the implementation?

This original study note follows the 1996/2022 [SICP comparison table of contents](https://sicp.sourceacademy.org/). The 1984 original may organize the subject differently.

[Parent section](../INTRO.md) · [Edition notes](../../../../docs/editions.md)
