# 2.2.1 Representing Sequences

## What this subsection teaches

Chaining pairs `(1, (2, (3, Nil)))` builds an ordered **sequence** accessed element by element. Decoupling sequence traversal from underlying storage lets `map`, `length`, and `append` work uniformly across arrays, linked chains, or lazy generators.

## Connecting the concept to Rust and Python

- **In Rust:** `Vec<T>`, slices `&[T]`, and iterators (`xs.iter().map(...)`) replace pointer chains with cache-friendly contiguous memory and bounds-checked indexing.
- **In Python:** Built-in `list` (dynamic array) and generator expressions `(f(x) for x in xs)` provide O(1) indexing and uniform iteration over any sequence (`list`, `tuple`, `range`, generator).
- **Why compare both:** Both Rust slices and Python lists favor contiguous arrays in practice, while iterator/generator pipelines preserve the single-pass compositional clarity of linked lists.

## Exercise

Trace `xs` reversed via `fold` and compute the sum of squared even numbers in a single sequence pipeline.

[Parent section](../INTRO.md) · [Chapter 2 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
