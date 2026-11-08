# 4.4.4 Implementing Queries

## What this subsection teaches

A query engine combines fact storage, unification, rule expansion, and controlled backtracking.

## Connecting the concept to Rust

Use a map of variable bindings and explicit fact-search strategies to represent relations. The translation is intended to explain computational ideas, not imitate the exact source language syntax. Think about the input, output, representation invariant, and what changes when evaluation order or mutable state enters the picture.

## Exercise

Join two parent relations to infer grandparents.

**Study method:** trace a small input on paper, run the closest example from this repository, then explain the result and resource usage. Which aspects are mathematical and which depend on the implementation?

This original study note follows the 1996/2022 [SICP comparison table of contents](https://sicp.sourceacademy.org/). The 1984 original may organize the subject differently.

[Parent section](../INTRO.md) · [Edition notes](../../../../docs/editions.md)
