# 3.3.1 Mutable List Structure

## What this subsection teaches

Pointers and mutation turn a list into a changing graph. Aliasing and cycles affect traversal and equality.

## Connecting the concept to Rust

Use VecDeque, HashMap, and optional Rc<RefCell<T>> sharing to model mutable structures. The translation is intended to explain computational ideas, not imitate the exact source language syntax. Think about the input, output, representation invariant, and what changes when evaluation order or mutable state enters the picture.

## Exercise

Draw a two-node cycle and explain nontermination of a naive walk.

**Study method:** trace a small input on paper, run the closest example from this repository, then explain the result and resource usage. Which aspects are mathematical and which depend on the implementation?

This original study note follows the 1996/2022 [SICP comparison table of contents](https://sicp.sourceacademy.org/). The 1984 original may organize the subject differently.

[Parent section](../INTRO.md) · [Edition notes](../../../../docs/editions.md)
