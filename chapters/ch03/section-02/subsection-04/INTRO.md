# 3.2.4 Internal Definitions

## What this subsection teaches

Mutually recursive local names require careful visibility and initialization rules. Simple sequential binding is not equivalent to recursive binding.

## Connecting the concept to Rust

Use a runtime HashMap and parent frames to model a language environment distinct from Rust lexical scope. The translation is intended to explain computational ideas, not imitate the exact source language syntax. Think about the input, output, representation invariant, and what changes when evaluation order or mutable state enters the picture.

## Exercise

Explain why a recursive function might need a placeholder binding.

**Study method:** trace a small input on paper, run the closest example from this repository, then explain the result and resource usage. Which aspects are mathematical and which depend on the implementation?

This original study note follows the 1996/2022 [SICP comparison table of contents](https://sicp.sourceacademy.org/). The 1984 original may organize the subject differently.

[Parent section](../INTRO.md) · [Edition notes](../../../../docs/editions.md)
