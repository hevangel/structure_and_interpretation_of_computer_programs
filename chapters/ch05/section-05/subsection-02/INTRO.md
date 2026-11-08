# 5.5.2 Compiling Expressions

## What this subsection teaches

Code generation emits instructions for operands and operators in an order that preserves source semantics.

## Connecting the concept to Rust

Use an expression enum, instruction vector, and virtual-machine stack to separate compilation from execution. The translation is intended to explain computational ideas, not imitate the exact source language syntax. Think about the input, output, representation invariant, and what changes when evaluation order or mutable state enters the picture.

## Exercise

Compile 2+3*4 into stack bytecode.

**Study method:** trace a small input on paper, run the closest example from this repository, then explain the result and resource usage. Which aspects are mathematical and which depend on the implementation?

This original study note follows the 1996/2022 [SICP comparison table of contents](https://sicp.sourceacademy.org/). The 1984 original may organize the subject differently.

[Parent section](../INTRO.md) · [Edition notes](../../../../docs/editions.md)
