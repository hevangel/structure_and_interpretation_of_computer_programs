# 3.1 Assignment and Local State

## The big idea

Track a balance across commands and observe how equal-looking functions can have different histories.

## What you should already know

You should be comfortable with basic functions, conditionals, and data structures. For this section, focus on **mutable accounts, local history, identity, random generators**. The important move is to separate the *interface* of a computational idea from the particular mechanism that realizes it.

## Translating the idea into Rust

Use &mut self or interior mutability deliberately; an object has an identity as well as a value.

Study the nearby examples, identify their data representation, and write down one invariant. Compare your prediction with the test assertions. Some original Scheme ideas are dynamically typed or depend on tail-call behavior; the Rust port deliberately highlights those differences rather than hiding them.

## Check your understanding

When does hidden state make a function harder to test?

**Experiment:** change the data representation or evaluation strategy without changing the observable API. Which tests survive?

[Back to book and edition notes](../../../docs/editions.md)
