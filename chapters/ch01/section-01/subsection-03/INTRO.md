# 1.1.3 Evaluating Combinations

## What this subsection teaches

A compound expression evaluates an operation and its arguments, then applies the operation. Evaluation order becomes visible when arguments have side effects.

## Connecting the concept to Rust

Use Rust let bindings, fn declarations, if expressions, and pure arithmetic functions to contrast naming with evaluation. The translation is intended to explain computational ideas, not imitate the exact source language syntax. Think about the input, output, representation invariant, and what changes when evaluation order or mutable state enters the picture.

## Exercise

Draw an evaluation tree for square(2+3)+square(4).

**Study method:** trace a small input on paper, run the closest example from this repository, then explain the result and resource usage. Which aspects are mathematical and which depend on the implementation?

This original study note follows the 1996/2022 [SICP comparison table of contents](https://sicp.sourceacademy.org/). The 1984 original may organize the subject differently.

[Parent section](../INTRO.md) · [Edition notes](../../../../docs/editions.md)
