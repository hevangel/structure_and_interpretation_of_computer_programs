# 4.4 Logic Programming

## The big idea

Resolve variables against facts and explore what declarative queries mean operationally.

## What you should already know

You should be comfortable with basic functions, conditionals, and data structures. For this section, focus on **facts, relations, variables, unification, query execution**. The important move is to separate the *interface* of a computational idea from the particular mechanism that realizes it.

## Translating the idea into Rust

Data structures and a unifier replace the implicit search machinery of a logic engine.

Study the nearby examples, identify their data representation, and write down one invariant. Compare your prediction with the test assertions. Some original Scheme ideas are dynamically typed or depend on tail-call behavior; the Rust port deliberately highlights those differences rather than hiding them.

## Check your understanding

What is the difference between stating a relation and choosing a search strategy?

**Experiment:** change the data representation or evaluation strategy without changing the observable API. Which tests survive?

[Back to book and edition notes](../../../docs/editions.md)
