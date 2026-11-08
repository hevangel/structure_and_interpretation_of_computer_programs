# 4.4.2 How Queries Work

## What this subsection teaches

Pattern matching and unification accumulate consistent bindings while search explores facts and derived relations.

## Connecting the concept to Rust

Use a map of variable bindings and explicit fact-search strategies to represent relations. The translation is intended to explain computational ideas, not imitate the exact source language syntax. Think about the input, output, representation invariant, and what changes when evaluation order or mutable state enters the picture.

## Exercise

Reject conflicting uses of the same variable.

**Study method:** trace a small input on paper, run the closest example from this repository, then explain the result and resource usage. Which aspects are mathematical and which depend on the implementation?

This original study note follows the 1996/2022 [SICP comparison table of contents](https://sicp.sourceacademy.org/). The 1984 original may organize the subject differently.

[Parent section](../INTRO.md) · [Edition notes](../../../../docs/editions.md)
