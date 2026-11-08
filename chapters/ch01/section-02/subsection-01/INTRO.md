# 1.2.1 Linear Recursion and Iteration

## What this subsection teaches

A recursive process stores deferred computation while an iterative process evolves bounded state. Recursive syntax does not guarantee constant-stack execution.

## Connecting the concept to Rust

Use unit tests and loops to compare process shapes. Rust does not guarantee optimization of tail-recursive function calls. The translation is intended to explain computational ideas, not imitate the exact source language syntax. Think about the input, output, representation invariant, and what changes when evaluation order or mutable state enters the picture.

## Exercise

Trace recursive and loop factorial on input four.

**Study method:** trace a small input on paper, run the closest example from this repository, then explain the result and resource usage. Which aspects are mathematical and which depend on the implementation?

This original study note follows the 1996/2022 [SICP comparison table of contents](https://sicp.sourceacademy.org/). The 1984 original may organize the subject differently.

[Parent section](../INTRO.md) · [Edition notes](../../../../docs/editions.md)
