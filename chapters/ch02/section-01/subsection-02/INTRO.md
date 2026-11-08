# 2.1.2 Abstraction Barriers

## What this subsection teaches

Representation choices belong behind constructors and selectors. Clients remain valid when storage changes as long as the public laws stay true.

## Connecting the concept to Rust

Use private struct fields, constructors, and invariants to enforce abstract data contracts. The translation is intended to explain computational ideas, not imitate the exact source language syntax. Think about the input, output, representation invariant, and what changes when evaluation order or mutable state enters the picture.

## Exercise

Change a private struct layout without changing tests.

**Study method:** trace a small input on paper, run the closest example from this repository, then explain the result and resource usage. Which aspects are mathematical and which depend on the implementation?

This original study note follows the 1996/2022 [SICP comparison table of contents](https://sicp.sourceacademy.org/). The 1984 original may organize the subject differently.

[Parent section](../INTRO.md) · [Edition notes](../../../../docs/editions.md)
