# 3.2.2 Applying Functions

## What this subsection teaches

A call introduces parameter bindings in a new environment and evaluates its body there. Closures must retain the right surrounding frame.

## Connecting the concept to Rust

Use a runtime HashMap and parent frames to model a language environment distinct from Rust lexical scope. The translation is intended to explain computational ideas, not imitate the exact source language syntax. Think about the input, output, representation invariant, and what changes when evaluation order or mutable state enters the picture.

## Exercise

Trace a call that refers to an outer variable.

**Study method:** trace a small input on paper, run the closest example from this repository, then explain the result and resource usage. Which aspects are mathematical and which depend on the implementation?

This original study note follows the 1996/2022 [SICP comparison table of contents](https://sicp.sourceacademy.org/). The 1984 original may organize the subject differently.

[Parent section](../INTRO.md) · [Edition notes](../../../../docs/editions.md)
