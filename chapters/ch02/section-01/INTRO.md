# 2.1 Introduction to Data Abstraction

## The big idea

Construct normalized rational numbers and make representation invariants unobservable to callers.

## What you should already know

You should be comfortable with basic functions, conditionals, and data structures. For this section, focus on **pairs, rational numbers, constructors/selectors, abstraction barriers**. The important move is to separate the *interface* of a computational idea from the particular mechanism that realizes it.

## Translating the idea into Rust

A private struct and public constructor can enforce invariants that Scheme relies on convention to preserve.

Study the nearby examples, identify their data representation, and write down one invariant. Compare your prediction with the test assertions. Some original Scheme ideas are dynamically typed or depend on tail-call behavior; the Rust port deliberately highlights those differences rather than hiding them.

## Check your understanding

If a rational's representation changed, which client functions should remain unchanged?

**Experiment:** change the data representation or evaluation strategy without changing the observable API. Which tests survive?

[Back to book and edition notes](../../../docs/editions.md)
