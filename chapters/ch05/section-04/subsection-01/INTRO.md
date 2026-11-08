# 5.4.1 Explicit-Control Evaluator Core

## What this subsection teaches

A control register and continuation frames replace implicit recursive host calls. Each step makes a small transition.

## Connecting the concept to Rust

Use control-state and continuation enums instead of nested host recursion. The translation is intended to explain computational ideas, not imitate the exact source language syntax. Think about the input, output, representation invariant, and what changes when evaluation order or mutable state enters the picture.

## Exercise

Trace evaluating a literal and returning the result.

**Study method:** trace a small input on paper, run the closest example from this repository, then explain the result and resource usage. Which aspects are mathematical and which depend on the implementation?

This original study note follows the 1996/2022 [SICP comparison table of contents](https://sicp.sourceacademy.org/). The 1984 original may organize the subject differently.

[Parent section](../INTRO.md) · [Edition notes](../../../../docs/editions.md)
