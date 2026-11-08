# 2.2 Hierarchical Data and the Closure Property

## The big idea

Represent a recursively branching structure and apply folds and maps. Understand why closure under composition matters.

## What you should already know

You should be comfortable with basic functions, conditionals, and data structures. For this section, focus on **lists, trees, mapping, sequences, combinatorial enumeration, picture language**. The important move is to separate the *interface* of a computational idea from the particular mechanism that realizes it.

## Translating the idea into Rust

Recursive data normally uses Box, Vec, or Rc because Rust values have known size.

Study the nearby examples, identify their data representation, and write down one invariant. Compare your prediction with the test assertions. Some original Scheme ideas are dynamically typed or depend on tail-call behavior; the Rust port deliberately highlights those differences rather than hiding them.

## Check your understanding

When is a tree walk a better abstraction than manual indexing?

**Experiment:** change the data representation or evaluation strategy without changing the observable API. Which tests survive?

[Back to book and edition notes](../../../docs/editions.md)
