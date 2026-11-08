# 4.1.3 Evaluator Data Structures

## What this subsection teaches

Values, environments, closures, and special forms need clear representations. This is the runtime model of the language being interpreted.

## Connecting the concept to Rust

Use an AST enum plus a separate evaluation function; the Rust interpreter is not literally metacircular. The translation is intended to explain computational ideas, not imitate the exact source language syntax. Think about the input, output, representation invariant, and what changes when evaluation order or mutable state enters the picture.

## Exercise

Define an enum for numbers, Booleans, and functions.

**Study method:** trace a small input on paper, run the closest example from this repository, then explain the result and resource usage. Which aspects are mathematical and which depend on the implementation?

This original study note follows the 1996/2022 [SICP comparison table of contents](https://sicp.sourceacademy.org/). The 1984 original may organize the subject differently.

[Parent section](../INTRO.md) · [Edition notes](../../../../docs/editions.md)
