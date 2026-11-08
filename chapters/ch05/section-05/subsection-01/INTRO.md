# 5.5.1 Compiler Structure

## What this subsection teaches

Compilation has stages: syntax analysis, instruction selection, linkage, and execution. Each stage has a separate contract.

## Connecting the concept to Rust

Use an expression enum, instruction vector, and virtual-machine stack to separate compilation from execution. The translation is intended to explain computational ideas, not imitate the exact source language syntax. Think about the input, output, representation invariant, and what changes when evaluation order or mutable state enters the picture.

## Exercise

Name the input and output of every stage.

**Study method:** trace a small input on paper, run the closest example from this repository, then explain the result and resource usage. Which aspects are mathematical and which depend on the implementation?

This original study note follows the 1996/2022 [SICP comparison table of contents](https://sicp.sourceacademy.org/). The 1984 original may organize the subject differently.

[Parent section](../INTRO.md) · [Edition notes](../../../../docs/editions.md)
