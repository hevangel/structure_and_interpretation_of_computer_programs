# 4.3 Variations on a Scheme: Nondeterministic Computing

## The big idea

Model alternatives and prune partial solutions through backtracking.

## What you should already know

You should be comfortable with basic functions, conditionals, and data structures. For this section, focus on **choice points, backtracking, constraint checking, search**. The important move is to separate the *interface* of a computational idea from the particular mechanism that realizes it.

## Translating the idea into Rust

Explicit recursion and Vec state make search branches easy to audit and test.

Study the nearby examples, identify their data representation, and write down one invariant. Compare your prediction with the test assertions. Some original Scheme ideas are dynamically typed or depend on tail-call behavior; the Rust port deliberately highlights those differences rather than hiding them.

## Check your understanding

How early can a constraint reject a branch without missing a solution?

**Experiment:** change the data representation or evaluation strategy without changing the observable API. Which tests survive?

[Back to book and edition notes](../../../docs/editions.md)
