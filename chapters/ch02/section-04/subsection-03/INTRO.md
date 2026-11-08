# 2.4.3 Data-Directed Programming

## What this subsection teaches

A table keyed by type and operation can make a system extensible without rewriting one central conditional for every addition.

## Connecting the concept to Rust

Use enum variants or traits to keep representation choices separate from abstract operations. The translation is intended to explain computational ideas, not imitate the exact source language syntax. Think about the input, output, representation invariant, and what changes when evaluation order or mutable state enters the picture.

## Exercise

Design a method table for two complex representations.

**Study method:** trace a small input on paper, run the closest example from this repository, then explain the result and resource usage. Which aspects are mathematical and which depend on the implementation?

This original study note follows the 1996/2022 [SICP comparison table of contents](https://sicp.sourceacademy.org/). The 1984 original may organize the subject differently.

[Parent section](../INTRO.md) · [Edition notes](../../../../docs/editions.md)
