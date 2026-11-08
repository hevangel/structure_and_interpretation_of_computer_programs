# 5.2.2 The Assembler

## What this subsection teaches

An assembler resolves symbolic labels into executable instruction addresses, usually in multiple passes.

## Connecting the concept to Rust

Use a program counter and a bounded execution loop to interpret machine instructions. The translation is intended to explain computational ideas, not imitate the exact source language syntax. Think about the input, output, representation invariant, and what changes when evaluation order or mutable state enters the picture.

## Exercise

Assemble a loop with two branches.

**Study method:** trace a small input on paper, run the closest example from this repository, then explain the result and resource usage. Which aspects are mathematical and which depend on the implementation?

This original study note follows the 1996/2022 [SICP comparison table of contents](https://sicp.sourceacademy.org/). The 1984 original may organize the subject differently.

[Parent section](../INTRO.md) · [Edition notes](../../../../docs/editions.md)
