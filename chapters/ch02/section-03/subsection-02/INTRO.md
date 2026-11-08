# 2.3.2 Symbolic Differentiation

## What this subsection teaches

Differentiation can transform an expression tree without evaluating it. Rewrite rules operate on sums, products, and variables.

## Connecting the concept to Rust

Use enum syntax trees and exhaustive match arms to manipulate symbols and trees. The translation is intended to explain computational ideas, not imitate the exact source language syntax. Think about the input, output, representation invariant, and what changes when evaluation order or mutable state enters the picture.

## Exercise

Differentiate x*x and evaluate its derivative at x=7.

**Study method:** trace a small input on paper, run the closest example from this repository, then explain the result and resource usage. Which aspects are mathematical and which depend on the implementation?

This original study note follows the 1996/2022 [SICP comparison table of contents](https://sicp.sourceacademy.org/). The 1984 original may organize the subject differently.

[Parent section](../INTRO.md) · [Edition notes](../../../../docs/editions.md)
