# 4.1 The Metacircular Evaluator

## The big idea

Implement a deliberately small evaluator whose syntax tree and evaluation rules are separate.

## What you should already know

You should be comfortable with basic functions, conditionals, and data structures. For this section, focus on **expression syntax, environments, application, evaluation order**. The important move is to separate the *interface* of a computational idea from the particular mechanism that realizes it.

## Translating the idea into Rust

A Rust interpreter is not metacircular in the literal Scheme sense; it illustrates the same evaluator architecture.

Study the nearby examples, identify their data representation, and write down one invariant. Compare your prediction with the test assertions. Some original Scheme ideas are dynamically typed or depend on tail-call behavior; the Rust port deliberately highlights those differences rather than hiding them.

## Check your understanding

Which features of the host language should the interpreted language inherit?

**Experiment:** change the data representation or evaluation strategy without changing the observable API. Which tests survive?

[Back to book and edition notes](../../../docs/editions.md)
