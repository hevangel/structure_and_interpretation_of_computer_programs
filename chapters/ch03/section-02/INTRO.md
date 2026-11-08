# 3.2 The Environment Model of Evaluation

## The big idea

Make environments first-class: lookups follow enclosing links while shadowing affects only the inner frame.

## What you should already know

You should be comfortable with basic functions, conditionals, and data structures. For this section, focus on **frames, variable lookup, lexical scope, closures, bindings**. The important move is to separate the *interface* of a computational idea from the particular mechanism that realizes it.

## Translating the idea into Rust

Rust's lexical lifetimes are a compile-time ownership concern; an interpreter's environments are runtime language values.

Study the nearby examples, identify their data representation, and write down one invariant. Compare your prediction with the test assertions. Some original Scheme ideas are dynamically typed or depend on tail-call behavior; the Rust port deliberately highlights those differences rather than hiding them.

## Check your understanding

Why does lexical scope depend on where a function is defined instead of called?

**Experiment:** change the data representation or evaluation strategy without changing the observable API. Which tests survive?

[Back to book and edition notes](../../../docs/editions.md)
