# 5.4.3 Conditionals and Declarations

## What this subsection teaches

A statement-capable evaluator needs explicit rules for branches, assignments, and declaration scope.

## Connecting the concept to Rust

Use control-state and continuation enums instead of nested host recursion. The translation is intended to explain computational ideas, not imitate the exact source language syntax. Think about the input, output, representation invariant, and what changes when evaluation order or mutable state enters the picture.

## Exercise

Trace a conditional that changes one binding.

**Study method:** trace a small input on paper, run the closest example from this repository, then explain the result and resource usage. Which aspects are mathematical and which depend on the implementation?

This original study note follows the 1996/2022 [SICP comparison table of contents](https://sicp.sourceacademy.org/). The 1984 original may organize the subject differently.

[Parent section](../INTRO.md) · [Edition notes](../../../../docs/editions.md)
