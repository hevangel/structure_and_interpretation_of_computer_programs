# 2.1.1 Rational Arithmetic

## What this subsection teaches

An abstract rational type should preserve a nonzero denominator and canonical sign. Client code should use arithmetic operations instead of touching fields.

## Connecting the concept to Rust

Use private struct fields, constructors, and invariants to enforce abstract data contracts. The translation is intended to explain computational ideas, not imitate the exact source language syntax. Think about the input, output, representation invariant, and what changes when evaluation order or mutable state enters the picture.

## Exercise

Normalize 2/-4 and compare it with -1/2.

**Study method:** trace a small input on paper, run the closest example from this repository, then explain the result and resource usage. Which aspects are mathematical and which depend on the implementation?

This original study note follows the 1996/2022 [SICP comparison table of contents](https://sicp.sourceacademy.org/). The 1984 original may organize the subject differently.

[Parent section](../INTRO.md) · [Edition notes](../../../../docs/editions.md)
