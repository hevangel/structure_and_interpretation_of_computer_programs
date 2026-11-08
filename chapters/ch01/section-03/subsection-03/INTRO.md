# 1.3.3 Functions as General Methods

## What this subsection teaches

Fixed-point iteration and numerical integration are reusable methods parameterized by problem-specific functions. Convergence is part of their contract.

## Connecting the concept to Rust

Use closures and the Fn traits to pass or return computations as values. The translation is intended to explain computational ideas, not imitate the exact source language syntax. Think about the input, output, representation invariant, and what changes when evaluation order or mutable state enters the picture.

## Exercise

Solve cos(x)=x and discuss divergence for another function.

**Study method:** trace a small input on paper, run the closest example from this repository, then explain the result and resource usage. Which aspects are mathematical and which depend on the implementation?

This original study note follows the 1996/2022 [SICP comparison table of contents](https://sicp.sourceacademy.org/). The 1984 original may organize the subject differently.

[Parent section](../INTRO.md) · [Edition notes](../../../../docs/editions.md)
