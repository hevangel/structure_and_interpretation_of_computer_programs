# 4.3.1 Amb and Search

## What this subsection teaches

Nondeterministic choice describes alternatives; backtracking is one strategy for exploring them. Failure restores a previous choice point.

## Connecting the concept to Rust

Use an explicit Vec of choices and recursive backtracking to model nondeterministic search. The translation is intended to explain computational ideas, not imitate the exact source language syntax. Think about the input, output, representation invariant, and what changes when evaluation order or mutable state enters the picture.

## Exercise

Solve two-digit constraints with explicit backtracking.

**Study method:** trace a small input on paper, run the closest example from this repository, then explain the result and resource usage. Which aspects are mathematical and which depend on the implementation?

This original study note follows the 1996/2022 [SICP comparison table of contents](https://sicp.sourceacademy.org/). The 1984 original may organize the subject differently.

[Parent section](../INTRO.md) · [Edition notes](../../../../docs/editions.md)
