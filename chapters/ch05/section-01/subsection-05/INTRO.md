# 5.1.5 Instruction Summary

## What this subsection teaches

Instruction sets must specify preconditions and how every opcode changes machine state. This operational model enables simulation.

## Connecting the concept to Rust

Use an instruction enum and named registers to design a small control machine. The translation is intended to explain computational ideas, not imitate the exact source language syntax. Think about the input, output, representation invariant, and what changes when evaluation order or mutable state enters the picture.

## Exercise

Write a transition rule for Load, Jump, and Halt.

**Study method:** trace a small input on paper, run the closest example from this repository, then explain the result and resource usage. Which aspects are mathematical and which depend on the implementation?

This original study note follows the 1996/2022 [SICP comparison table of contents](https://sicp.sourceacademy.org/). The 1984 original may organize the subject differently.

[Parent section](../INTRO.md) · [Edition notes](../../../../docs/editions.md)
