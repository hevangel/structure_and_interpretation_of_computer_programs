# 3.1.1 Local State Variables

## What this subsection teaches

An object may remember information between calls. Two separately constructed counters can share code but have independent histories.

## Connecting the concept to Rust

Use mutable structs or closures and observe how state affects later calls. The translation is intended to explain computational ideas, not imitate the exact source language syntax. Think about the input, output, representation invariant, and what changes when evaluation order or mutable state enters the picture.

## Exercise

Interleave calls to two counters and predict results.

**Study method:** trace a small input on paper, run the closest example from this repository, then explain the result and resource usage. Which aspects are mathematical and which depend on the implementation?

This original study note follows the 1996/2022 [SICP comparison table of contents](https://sicp.sourceacademy.org/). The 1984 original may organize the subject differently.

[Parent section](../INTRO.md) · [Edition notes](../../../../docs/editions.md)
