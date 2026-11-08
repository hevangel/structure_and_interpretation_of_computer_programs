# 1.2.2 Tree Recursion

## What this subsection teaches

Branching recurrences expose repeated subproblems. A call tree helps distinguish concise source code from expensive execution.

## Connecting the concept to Rust

Use unit tests and loops to compare process shapes. Rust does not guarantee optimization of tail-recursive function calls. The translation is intended to explain computational ideas, not imitate the exact source language syntax. Think about the input, output, representation invariant, and what changes when evaluation order or mutable state enters the picture.

## Exercise

Draw the recursion tree of fib(5) and identify duplicate work.

**Study method:** trace a small input on paper, run the closest example from this repository, then explain the result and resource usage. Which aspects are mathematical and which depend on the implementation?

This original study note follows the 1996/2022 [SICP comparison table of contents](https://sicp.sourceacademy.org/). The 1984 original may organize the subject differently.

[Parent section](../INTRO.md) · [Edition notes](../../../../docs/editions.md)
