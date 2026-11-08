# 5.1.4 A Stack for Recursion

## What this subsection teaches

A machine stack records suspended computations, parameters, temporary data, and return positions.

## Connecting the concept to Rust

Use an instruction enum and named registers to design a small control machine. The translation is intended to explain computational ideas, not imitate the exact source language syntax. Think about the input, output, representation invariant, and what changes when evaluation order or mutable state enters the picture.

## Exercise

Trace factorial stack frames for n=4.

**Study method:** trace a small input on paper, run the closest example from this repository, then explain the result and resource usage. Which aspects are mathematical and which depend on the implementation?

This original study note follows the 1996/2022 [SICP comparison table of contents](https://sicp.sourceacademy.org/). The 1984 original may organize the subject differently.

[Parent section](../INTRO.md) · [Edition notes](../../../../docs/editions.md)
