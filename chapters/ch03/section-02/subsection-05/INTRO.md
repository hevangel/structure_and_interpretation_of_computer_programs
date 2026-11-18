# 3.2.5 The CSE Machine

## What this subsection teaches

The JavaScript adaptation makes control, stack, and environment explicit to explain evaluation step by step.

## Connecting the concept to Rust

Use a runtime HashMap and parent frames to model a language environment distinct from Rust lexical scope. The translation is intended to explain computational ideas, not imitate the exact source language syntax. Think about the input, output, representation invariant, and what changes when evaluation order or mutable state enters the picture.

## Exercise

Trace the machine state for 1+2*3.

**Study method:** trace a small input on paper, run the closest example from this repository, then explain the result and resource usage. Which aspects are mathematical and which depend on the implementation?

This original study note follows the 1996/2022 [SICP comparison table of contents](https://sicp.sourceacademy.org/). Note that **3.2.5 The CSE Machine** is a Source Academy web-comparison supplement rather than a numbered subsection in the printed 1996 Scheme or 2022 JavaScript textbook volumes (which both have 94 numbered subsections, `1.1.1`–`5.5.7`). The 1984 original may organize the subject differently.

[Parent section](../INTRO.md) · [Edition notes](../../../../docs/editions.md)
