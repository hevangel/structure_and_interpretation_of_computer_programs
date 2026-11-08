# 2.4 Multiple Representations for Abstract Data

## The big idea

Implement alternative internal representations behind the same operations.

## What you should already know

You should be comfortable with basic functions, conditionals, and data structures. For this section, focus on **complex numbers, tags, message passing, dispatch**. The important move is to separate the *interface* of a computational idea from the particular mechanism that realizes it.

## Translating the idea into Rust

An enum is one dispatch choice; traits let third parties implement a common interface.

Study the nearby examples, identify their data representation, and write down one invariant. Compare your prediction with the test assertions. Some original Scheme ideas are dynamically typed or depend on tail-call behavior; the Rust port deliberately highlights those differences rather than hiding them.

## Check your understanding

Which operations belong in the public abstraction rather than in a representation?

**Experiment:** change the data representation or evaluation strategy without changing the observable API. Which tests survive?

[Back to book and edition notes](../../../docs/editions.md)
