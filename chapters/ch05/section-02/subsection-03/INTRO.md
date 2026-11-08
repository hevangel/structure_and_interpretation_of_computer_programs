# 5.2.3 Instruction Execution Functions

## What this subsection teaches

A simulator's dispatcher implements each opcode and reports invalid operands or jumps rather than silently corrupting state.

## Connecting the concept to Rust

Use a program counter and a bounded execution loop to interpret machine instructions. The translation is intended to explain computational ideas, not imitate the exact source language syntax. Think about the input, output, representation invariant, and what changes when evaluation order or mutable state enters the picture.

## Exercise

Test an out-of-range register operand.

**Study method:** trace a small input on paper, run the closest example from this repository, then explain the result and resource usage. Which aspects are mathematical and which depend on the implementation?

This original study note follows the 1996/2022 [SICP comparison table of contents](https://sicp.sourceacademy.org/). The 1984 original may organize the subject differently.

[Parent section](../INTRO.md) · [Edition notes](../../../../docs/editions.md)
