# 3.3.3 Representing Tables

## What this subsection teaches

Tables associate keys with values and can represent nested indexing. Missing and default values must be distinguished.

## Connecting the concept to Rust

Use VecDeque, HashMap, and optional Rc<RefCell<T>> sharing to model mutable structures. The translation is intended to explain computational ideas, not imitate the exact source language syntax. Think about the input, output, representation invariant, and what changes when evaluation order or mutable state enters the picture.

## Exercise

Implement a table indexed by a pair of keys.

**Study method:** trace a small input on paper, run the closest example from this repository, then explain the result and resource usage. Which aspects are mathematical and which depend on the implementation?

This original study note follows the 1996/2022 [SICP comparison table of contents](https://sicp.sourceacademy.org/). The 1984 original may organize the subject differently.

[Parent section](../INTRO.md) · [Edition notes](../../../../docs/editions.md)
