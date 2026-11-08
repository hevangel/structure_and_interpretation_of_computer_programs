# 3.2.3 Frames and Local State

## What this subsection teaches

An environment frame stores local bindings and points to an enclosing frame. Shadowing introduces a new binding rather than overwriting every ancestor.

## Connecting the concept to Rust

Use a runtime HashMap and parent frames to model a language environment distinct from Rust lexical scope. The translation is intended to explain computational ideas, not imitate the exact source language syntax. Think about the input, output, representation invariant, and what changes when evaluation order or mutable state enters the picture.

## Exercise

Draw two frames that both bind x.

**Study method:** trace a small input on paper, run the closest example from this repository, then explain the result and resource usage. Which aspects are mathematical and which depend on the implementation?

This original study note follows the 1996/2022 [SICP comparison table of contents](https://sicp.sourceacademy.org/). The 1984 original may organize the subject differently.

[Parent section](../INTRO.md) · [Edition notes](../../../../docs/editions.md)
