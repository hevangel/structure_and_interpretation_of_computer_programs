# 1.1 The Elements of Programming

## The big idea

Build a square-root solver by repeatedly improving a guess. Treat each function as an abstraction with an input and a contract.

## What you should already know

You should be comfortable with basic functions, conditionals, and data structures. For this section, focus on **expressions, naming, conditional evaluation, and newton iteration**. The important move is to separate the *interface* of a computational idea from the particular mechanism that realizes it.

## Translating the idea into Rust

An immutable binding is not the same as a mutable variable; distinguish evaluating an expression from assigning to a location.

Study the nearby examples, identify their data representation, and write down one invariant. Compare your prediction with the test assertions. Some original Scheme ideas are dynamically typed or depend on tail-call behavior; the Rust port deliberately highlights those differences rather than hiding them.

## Check your understanding

How would you prove your stopping rule is good enough for small and large numbers?

**Experiment:** change the data representation or evaluation strategy without changing the observable API. Which tests survive?

[Back to book and edition notes](../../../docs/editions.md)
